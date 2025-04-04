use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::sync::Arc;
use rustls::{
    ServerConfig, pki_types::{CertificateDer, PrivateKeyDer},
};
use rustls_pemfile::{certs, pkcs8_private_keys};
use serde_json;
use tokio::sync::{broadcast, Mutex};
use tokio_rustls::TlsAcceptor;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use tracing::{debug, error, info, warn};

mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // подключаемся к базе
    let db_pool = db::create_db_pool().await?;
    info!("Подключение к базе данных успешно");

    // Загрузка сертификата и ключа для TLS
    let cert_file = &mut BufReader::new(fs::File::open("cert.pem")?);
    let key_file = &mut BufReader::new(fs::File::open("key.pem")?);
    
    // парсим сертификаты
    let certs: Vec<CertificateDer<'_>> = certs(cert_file)
        .map(|result| result.map(CertificateDer::from))
        .collect::<Result<Vec<_>, _>>()?;

    // парсим закрытый ключ
    let keys: Vec<PrivateKeyDer<'_>> = pkcs8_private_keys(key_file)
        .map(|result| {
            result.map(|key| PrivateKeyDer::from(PrivateKeyDer::Pkcs8(key)))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if keys.is_empty() {
        error!("Закрытый ключ не найден");
        return Err("Закрытый ключ не найден".into());
    }

    // Создаём конфигурацию сервера
    let config = ServerConfig::builder()
        .with_no_client_auth() // не требуем аутентификации клиента
        .with_single_cert(certs, keys[0].clone_key())?; // Используем первый закрытый ключ

    let acceptor = TlsAcceptor::from(Arc::new(config));

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

        // Принимаем TLS соединение
        let acceptor = acceptor.clone();
        let mut tls_stream = match acceptor.accept(stream).await {
            Ok(stream) => stream,
            Err(e) => {
                error!("Ошибка при установке TLS-соединения: {}", e);
                continue;
            }
        };

        // Клонируем состояние для каждой задачи
        let clients = clients.clone();
        let db_pool = db_pool.clone();
        let tx = tx.clone();

        // Обрабатываем подключение в отдельной задаче
        tokio::spawn(async move {
            let mut buffer = [0; 1024]; // Буфер для чтения данных
            let mut username = None;

            // подписываемся на получение сообщений
            let mut rx = tx.subscribe();

            // авторизация клиента
            let authenticated = autentificate_client(&mut tls_stream).await;
            if !authenticated {
                warn!("Клиент {} не прошел авторизацию", addr);
                return;
            }

            // отправляем историю сообщений новому клиенту
            let history = db::load_history(&db_pool, 10).await.unwrap_or_default();
            for (sender, content) in history {
                let message = Message::ReceiveMessage {
                    sender: sender.clone(),
                    content: content.clone(),
                };
                send_massage(&mut tls_stream, &message).await;
            }

            loop {
                tokio::select! {
                    // чтение данных от клиента
                    result = tls_stream.read(&mut buffer) => {
                        match result {
                            // клиент отключился
                            Ok(n) if n == 0 => break,
                            // Успешно прочитано n байт
                            Ok(n) => {
                                debug!("Получено {} байт данных от клиента", n);
                                // Преобразуем байты в строку
                                let message_str = match String::from_utf8(buffer[..n].to_vec()) {
                                    Ok(s) => s,
                                    Err(_) => {
                                        error!("Ошибка декодирования utf-8");
                                        continue;
                                    }
                                };

                                // Десериализуем JSON в структуру Message
                                let message: Message = match serde_json::from_str(&message_str) {
                                    Ok(msg) => msg,
                                    Err(e) => {
                                        error!("Ошибка десериализации JSON: {}", e);
                                        continue;
                                    }
                                };

                                // Обрабатываем сообщение
                                match message {
                                    Message::Register { username, password } => {
                                        match db::register_user(&db_pool, &username, &password).await {
                                            Ok(_) => {
                                                let response = Message::ReceiveMessage {
                                                    sender: "Server".to_string(),
                                                    content: "Регистрация успешна".to_string(),
                                                };
                                                send_massage(&mut tls_stream, &response).await;
                                            }
                                            Err(e) => {
                                                let response = Message::ErrorMessage {
                                                    error: e.to_string(),
                                                };
                                                send_massage(&mut tls_stream, &response).await;
                                            }
                                        }
                                    }
                                    Message::Authenticate { username, password } => {
                                        match db::authenticate_user(&db_pool, &username, &password).await {
                                            Ok(true) => {
                                                let response = Message::ReceiveMessage {
                                                    sender: "Server".to_string(),
                                                    content: "Авторизация успешна".to_string(),
                                                };
                                                send_massage(&mut tls_stream, &response).await;
                                            }
                                            Ok(false) => {}
                                            Err(e) => {}
                                        }
                                    }
                                    Message::Join { username: new_username } => {
                                        info!("Клиент {} клиент пытается присоединиться", new_username);

                                        // Проверяем свободно ли имя
                                        let mut clients_lock = clients.lock().await;
                                        if clients_lock.contains_key(&new_username) {
                                            drop(clients_lock);

                                            let error_message = Message::ErrorMessage {
                                                error: format!("Имя {} уже занято", new_username),
                                            };
                                            send_massage(&mut tls_stream, &error_message).await;
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
                                        send_massage(&mut tls_stream, &response).await;

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
                                            if let Err(e) = db::save_message(&db_pool, sender, &content).await {
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
                                                send_massage(&mut tls_stream, &error_message).await;
                                                warn!("Клиент {} попытался отправить сообщение не существующему пользователю {}", sender, recipient);
                                            };
                                        };

                                    }
                                    _ => {}
                                }
                            }
                            Err(e) => {
                                error!("Ошибка чтения от клиента {}: {}", addr, e);
                                break;
                            }
                        }
                    }

                    // получаем сообщения из канала
                    result = rx.recv() => {
                        match result {
                            Ok(msg) => {
                                if let Err(e) = tls_stream.write_all(msg.as_bytes()).await {
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
    Authenticate { username: String, password: String }, // сообщение на авторизацию
    Join { username: String }, // Клиент присоединяется к чату
    SendMessage { content: String }, // Клиент отправляет сообщение
    ReceiveMessage { sender: String, content: String }, // Сообщение для клиента,
    SendPrivateMessage { recipient: String, content: String }, // Отправка приватных сообщений
    ReceivePrivateMessage { sender: String, content: String }, // Получение приватных сообщений
    Leave, // выход пользователя
    ErrorMessage { error: String }, // Ответ об ошибке
    Register { username: String, password: String }, // регистрация
}

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

async fn send_massage(stream: &mut tokio_rustls::server::TlsStream<tokio::net::TcpStream>, message: &Message) {
    let json_message = serde_json::to_string(message).unwrap();
    if let Err(e) = stream.write_all(json_message.as_bytes()).await {
        error!("Ошибка записи {}", e);
    }
}

// функция для авторизации клиента
async fn autentificate_client(stream: &mut tokio_rustls::server::TlsStream<tokio::net::TcpStream>) -> bool {
    let mut buffer = [0; 1024];

    // читаем данные от клиента
    let n = match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => return false, // клиент отключился
        Ok(n) => n,
        Err(_) => return false,
    };

    // преобразуем байты в строку
    let message_str = match String::from_utf8(buffer[..n].to_vec()) {
        Ok(s) => s,
        Err(_) => return false,
    };
    
    // десереализуем JSON в структуру Message
    let message: Message = match serde_json::from_str(&message_str) {
        Ok(msg) => msg,
        Err(_) => return false,
    };

    // проверяем учетные данные
    match message {
        Message::Authenticate { username, password } => {
            if username == "admin" && password == "password" {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}
