use std::collections::HashMap;
use futures_util::StreamExt;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message as WsMessage;
use futures_util::sink::SinkExt;
use std::sync::Arc;
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use handlers::{auth, chat};
use serde_json;
use tokio::sync::{broadcast, Mutex};
use tokio::net::TcpListener;
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info, warn};
use types::{AppResult, DbPool};
use crate::db::{db_main, group_chat, messages};

mod db;
mod services;
mod handlers;
mod types;
mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // подключаемся к базе
    let db_pool = db_main::create_db_pool().await?;
    info!("Подключение к базе данных успешно");
    let db_pool = Arc::new(db_pool);

    // // Загрузка сертификата и ключа для TLS
    // let cert_file = &mut BufReader::new(fs::File::open("cert.pem")?);
    // let key_file = &mut BufReader::new(fs::File::open("key.pem")?);

    // // парсим сертификаты
    // let certs: Vec<CertificateDer<'_>> = certs(cert_file)
    //     .map(|result| result.map(CertificateDer::from))
    //     .collect::<Result<Vec<_>, _>>()?;

    // // парсим закрытый ключ
    // let keys: Vec<PrivateKeyDer<'_>> = pkcs8_private_keys(key_file)
    //     .map(|result| {
    //         result.map(|key| PrivateKeyDer::from(PrivateKeyDer::Pkcs8(key)))
    //     })
    //     .collect::<Result<Vec<_>, _>>()?;

    // if keys.is_empty() {
    //     error!("Закрытый ключ не найден");
    //     return Err("Закрытый ключ не найден".into());
    // }

    // // Создаём конфигурацию сервера
    // let config = ServerConfig::builder()
    //     .with_no_client_auth() // не требуем аутентификации клиента
    //     .with_single_cert(certs, keys[0].clone_key())?; // Используем первый закрытый ключ

    // let acceptor = TlsAcceptor::from(Arc::new(config));

    let http_db_pool = Arc::clone(&db_pool);
    tokio::spawn(async move {
        HttpServer::new(move || {
            let cors = Cors::default()
                .send_wildcard()
                // .allowed_origin("http://localhost:8082")
                .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::ACCEPT,
                    actix_web::http::header::CONTENT_TYPE,
                ])
                .max_age(3600); // Время жизни предварительного запроса (preflight)

            App::new()
                .wrap(cors)
                .app_data(web::Data::new(http_db_pool.clone()))
                .route("/register", web::post().to(auth::register))
                .route("/login", web::post().to(auth::login))
                .route("/chats", web::post().to(chat::create))
                .route("/chats", web::delete().to(chat::delete))
                .route("/chats", web::get().to(chat::get_all))
                .route("/chats", web::route().guard(actix_web::guard::Options()).to(|| async {
                    info!("Предварительный запрос OPTIONS обработан");
                    HttpResponse::Ok()
                }))
        })
        .bind("127.0.0.1:8081")
        .unwrap()
        .run()
        .await
        .unwrap();
    });

    // Создаем TCP-слушатель на порту 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Сервер запущен на 127.0.0.1:8080");

    // создаем канал для рассылки сообщений
    let (tx, _) = broadcast::channel(32);

    // глобальное состояние сервера
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // Принимаем входящее подключение
        let (stream, addr) = listener.accept().await?;
        info!("Новое подключение: {}", addr);

        // // Принимаем TLS соединение
        // let acceptor = acceptor.clone();
        // let mut ws_stream = match acceptor.accept(stream).await {
        //     Ok(stream) => stream,
        //     Err(e) => {
        //         error!("Ошибка при установке TLS-соединения: {}", e);
        //         continue;
        //     }
        // };

        // Принимаем WebSocket-соединение
        let mut ws_stream = match accept_async(stream).await {
            Ok(ws_stream) => ws_stream,
            Err(e) => {
                error!("Ошибка при установке WebSocket-соединения: {}", e);
                continue;
            }
        };

        // Клонируем состояние для каждой задачи
        let clients = clients.clone();
        let db_pool = db_pool.clone();
        let tx = tx.clone();

        // Обрабатываем подключение в отдельной задаче
        tokio::spawn(async move {
            // let mut buffer = [0; 1024]; // Буфер для чтения данных
            let mut username = None;

            // подписываемся на получение сообщений
            let mut rx = tx.subscribe();

            // отправляем историю сообщений новому клиенту
            let history = messages::load_history(&db_pool, 10).await.unwrap_or_default();
            for (sender, content) in history {
                let message = Message::ReceiveMessage {
                    sender: sender.clone(),
                    content: content.clone(),
                };
                if let Err(e) = ws_stream.send(WsMessage::Text(serde_json::to_string(&message).unwrap().into())).await {
                    error!("Ошибка отправки истории {}", e);
                    return;
                }
                // send_massage(&mut ws_stream, &message).await;
            }

            loop {
                tokio::select! {
                    // чтение данных от клиента
                    result = ws_stream.next() => {
                        match result {
                            // Успешно прочитано сообщение
                            Some(Ok(message)) => {
                                match message {
                                    WsMessage::Text(text) => {
                                        debug!("Получено сообщение от клиента: {}", text);

                                        // Десериализуем JSON в структуру Message
                                        let message: Message = match serde_json::from_str(&text) {
                                            Ok(msg) => msg,
                                            Err(e) => {
                                                error!("Ошибка десериализации JSON: {}", e);
                                                continue;
                                            }
                                        };

                                        // Обрабатываем сообщение
                                        match message {
                                            Message::Join { username: new_username } => {
                                                info!("Клиент {} клиент пытается присоединиться", new_username);

                                                // Проверяем свободно ли имя
                                                let mut clients_lock = clients.lock().await;
                                                if clients_lock.contains_key(&new_username) {
                                                    drop(clients_lock);

                                                    let error_message = Message::ErrorMessage {
                                                        error: format!("Имя {} уже занято", new_username),
                                                    };
                                                    send_massage(&mut ws_stream, &error_message).await;
                                                    warn!("Клиент {} попытался присоединиться с занятым именем", new_username);
                                                    break;
                                                }

                                                // добавляем клиента в список
                                                clients_lock.insert(new_username.clone(), tx.clone());
                                                drop(clients_lock);

                                                username = Some(new_username.clone());

                                                // отправляем приветственное сообщение
                                                let response = Message::ReceiveMessage {
                                                    sender: "Server".to_string(),
                                                    content: format!("Добро пожаловать {}!", new_username),
                                                };
                                                send_massage(&mut ws_stream, &response).await;

                                                // уведомляем других участников о новом клиенте
                                                let notification = Message::ReceiveMessage {
                                                    sender: "Server".to_string(),
                                                    content: format!("{} присоединился к чату", new_username),
                                                };
                                                let notification_json = serde_json::to_string(&notification).unwrap();
                                                tx.send(notification_json).unwrap();
                                            }
                                            Message::SendMessage { content } => {
                                                if let Some(sender) = &username {
                                                    info!("Получено сообщение от {}: {}", sender, content);
                                                    // сохраняем сообщение в базу данных
                                                    if let Err(e) = messages::save_message(&db_pool, sender, &content).await {
                                                        error!("Ошибка загрузки сообщения в базу данных: {}", e);
                                                    };
                                                    let message = Message::ReceiveMessage {
                                                        sender: sender.clone(),
                                                        content: content.clone(),
                                                    };
                                                    let message_json = serde_json::to_string(&message).unwrap();

                                                    if let Err(e) = tx.send(message_json) {
                                                        error!("Ошибка отправки в канал: {}", e);
                                                    }
                                                }
                                            }
                                            Message::Leave => {
                                                if let Some(sender) = &username {
                                                    info!("Клиент {} покидает чат", sender);

                                                    // оповещаем других участников о выходе клиента
                                                    let notification = Message::ReceiveMessage {
                                                        sender: "Server".to_string(),
                                                        content: format!("{} покинул чат", sender),
                                                    };
                                                    let notification_json = serde_json::to_string(&notification).unwrap();
                                                    tx.send(notification_json).unwrap();

                                                    // удаляем клиента из списка
                                                    let mut clients_lock = clients.lock().await;
                                                    clients_lock.remove(sender);
                                                    drop(clients_lock);

                                                    // завершение задачи для этого клиента
                                                    break;
                                                }
                                            }
                                            Message::SendPrivateMessage { recipient, content } => {
                                                if let Some(sender) = &username {
                                                    info!("Приватное сообщение от {} для {}: {}", sender, recipient, content);

                                                    // находим получателя
                                                    let clients_lock = clients.lock().await;
                                                    if let Some(recipient_sender) = clients_lock.get(&recipient) {
                                                        // отправляем сообщение получателю
                                                        let private_message = Message::ReceivePrivateMessage {
                                                            sender: sender.clone(),
                                                            content: format!("[Приватно] {}", content),
                                                        };
                                                        let private_message_json = serde_json::to_string(&private_message).unwrap();
                                                        if let Err(e) = recipient_sender.send(private_message_json) {
                                                            error!("Ошибка отправки приватного сообщения клиенту {}: {}", recipient, e);
                                                        }
                                                    } else {
                                                        // отправитель не найден
                                                        drop(clients_lock);
                                                        let error_message = Message::ErrorMessage {
                                                            error: format!("Пользователь {} не найден", recipient),
                                                        };
                                                        send_massage(&mut ws_stream, &error_message).await;
                                                        warn!("Клиент {} попытался отправить сообщение не существующему пользователю {}", sender, recipient);
                                                    };
                                                };

                                            }
                                            Message::AddMemberToGroupChat { chat_id, username } => {
                                                match  group_chat::add_member(&db_pool, chat_id, &username).await {
                                                    Ok(_) => {
                                                        let response = Message::ReceiveMessage {
                                                            sender: "Server".to_string(),
                                                            content: format!("Участник '{}' успешно добавлен в групповой чат ID: {}", username, chat_id),
                                                        };
                                                        send_massage(&mut ws_stream, &response).await;
                                                    }
                                                    Err(e) => {
                                                        let response = Message::ErrorMessage {
                                                            error: e.to_string(),
                                                        };
                                                        send_massage(&mut ws_stream, &response).await;
                                                    }
                                                }
                                            }
                                            Message::SendMessageToGroupChat { chat_id, content } => {
                                                if let Some(sender_username) = &username {
                                                    match send_message_to_group_chat(&mut ws_stream, &db_pool, chat_id, sender_username, &content).await {
                                                        Ok(_) => {
                                                            info!("Сообщение '{}' успешно отправлено в групповой чат ID: {}", content, chat_id);
                                                        }
                                                        Err(e) => {
                                                            error!("Ошибка отправки сообщения в групповой чат {}", e);
                                                            let response = Message::ErrorMessage {
                                                                error: e.to_string(),
                                                            };
                                                            send_massage(&mut ws_stream, &response).await;
                                                        }
                                                    }
                                                } else {
                                                    warn!("Попытка отправить сообщение без авторизации");
                                                }
                                            }
                                            Message::RemoveMemberFromGroupChat { chat_id, username, requester } => {
                                                match group_chat::remove_member(&db_pool, chat_id, &username, &requester).await {
                                                    Ok(_) => {
                                                        let response = Message::ReceiveMessage {
                                                            sender: "Server".to_string(),
                                                            content: format!("Участник {} удален из группового чата ID: {}", username, chat_id),
                                                        };
                                                        send_massage(&mut ws_stream, &response).await;
                                                    }
                                                    Err(e) => {
                                                        error!("Ошибка удаления пользователя из группового чата {}", e);
                                                        let response = Message::ErrorMessage {
                                                            error: e.to_string(),
                                                        };
                                                        send_massage(&mut ws_stream, &response).await;
                                                    }
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    WsMessage::Close(_) => {
                                        info!("Клиент закрыл соединение!");
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            Some(Err(e)) => {
                                error!("Ошибка чтения от клиента {}: {}", addr, e);
                                break;
                            }
                            None => {
                                info!("Клиент отключился!");
                                break;
                            }
                        }
                    }

                    // получаем сообщения из канала
                    result = rx.recv() => {
                        match result {
                            Ok(msg) => {
                                if let Err(e) = ws_stream.send(WsMessage::Text(msg.into())).await {
                                    error!("Ошибка записи: {}", e);
                                    break;
                                } 
                            }
                            Err(e) => {
                                error!("Ошибка получения из канала: {}", e);
                                break;
                            }
                        }
                    }
                }
            }

            if let Some(username) = username {
                let mut clients_lock = clients.lock().await;
                clients_lock.remove(&username);
                info!("Клиент {} отключился", username);
            }
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")] // Указываем поле `type` для различения типов сообщений
enum Message {
    Join { username: String }, // Клиент присоединяется к чату
    SendMessage { content: String }, // Клиент отправляет сообщение
    ReceiveMessage { sender: String, content: String }, // Сообщение для клиента,
    SendPrivateMessage { recipient: String, content: String }, // Отправка приватных сообщений
    ReceivePrivateMessage { sender: String, content: String }, // Получение приватных сообщений
    Leave, // выход пользователя
    ErrorMessage { error: String }, // Ответ об ошибке
    AddMemberToGroupChat { chat_id: i32, username: String }, // добавить пользователя в групповой чат
    SendMessageToGroupChat { chat_id: i32, content: String }, // отправить сообщение в группвой чат
    ReceiveGroupChatMessage { chat_id: i32, sender: String, content: String }, // получение соощения из группового чата
    RemoveMemberFromGroupChat { chat_id: i32, username: String, requester: String }, // удалить пользователя из чата
}

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

// async fn send_massage(stream: &mut tokio_rustls::server::TlsStream<tokio::net::TcpStream>, message: &Message) {
//     let json_message = serde_json::to_string(message).unwrap();
//     if let Err(e) = stream.write_all(json_message.as_bytes()).await {
//         error!("Ошибка записи {}", e);
//     }
// }

async fn send_massage(stream: &mut WebSocketStream<tokio::net::TcpStream>, message: &Message) {
    let json_message = serde_json::to_string(message).unwrap();
    if let Err(e) = stream.send(WsMessage::Text(json_message.into())).await {
        error!("Ошибка записи {}", e);
    }
}

async fn send_message_to_group_chat(
    stream: &mut WebSocketStream<tokio::net::TcpStream>,
    db_pool: &DbPool,
    chat_id: i32,
    sender: &str,
    content: &str,
) -> AppResult<()> {
    let members = group_chat::get_members(db_pool, chat_id).await?;
    let message = Message::ReceiveGroupChatMessage {
        chat_id, sender: sender.to_string(), content: content.to_string(),
    };

    for _ in members {
        send_massage(stream, &message).await;
    }

    Ok(())
}
