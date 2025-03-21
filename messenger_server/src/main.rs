use std::collections::HashMap;
use std::sync::Arc;
use serde_json;
use tokio::sync::{broadcast, Mutex};

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // настройка логирования
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Создаем TCP-слушатель на порту 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!("Сервер запущен на 127.0.0.1:8080");

    // создаем канал для рассылки сообщений
    let (tx, _) = broadcast::channel(32);

    // глобальное состояние сервера
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // Принимаем входящее подключение
        let (mut socket, addr) = listener.accept().await?;
        info!("Новое подключение: {}", addr);

        // Клонируем состояние для каждой задачи
        let clients = clients.clone();
        let tx = tx.clone();

        // Обрабатываем подключение в отдельной задаче
        tokio::spawn(async move {
            let mut buffer = [0; 1024]; // Буфер для чтения данных
            let mut username = None;

            // подписываемся на получение сообщений
            let mut rx = tx.subscribe();

            loop {
                tokio::select! {
                    // чтение данных от клиента
                    result = socket.read(&mut buffer) => {
                        // Читаем данные из сокета
                        let n = match result {
                            Ok(n) if n == 0 => {
                                // Клиент закрыл соединение
                                println!("Клиент {} отключился", addr);
                                return;
                            }
                            Ok(n) => n, // Успешно прочитано n байт
                            Err(e) => {
                                eprintln!("Ошибка чтения от клиента {}: {}", addr, e);
                                return;
                            }
                        };

                        // Преобразуем байты в строку
                        let message_str = match String::from_utf8(buffer[..n].to_vec()) {
                            Ok(s) => s,
                            Err(_) => {
                                eprintln!("Ошибка декодирования utf-8");
                                continue;
                            }
                        };

                        // Десериализуем JSON в структуру Message
                        let message: Message = match serde_json::from_str(&message_str) {
                            Ok(msg) => msg,
                            Err(e) => {
                                eprintln!("Ошибка десериализации JSON: {}", e);
                                continue;
                            }
                        };

                        // Обрабатываем сообщение
                        match message {
                            Message::Join { username: new_username } => {
                                println!("Клиент {} клиент пытается присоединиться", new_username);

                                // Проверяем свободно ли имя
                                let mut clients_lock = clients.lock().await;
                                if clients_lock.contains_key(&new_username) {
                                    drop(clients_lock);

                                    let error_message = Message::ErrorMessage {
                                        error: format!("Имя {} уже занято", new_username),
                                    };
                                    send_massage(&mut socket, &error_message).await;
                                    break;
                                }

                                // добавляем клиента в список
                                clients_lock.insert(new_username.clone(), tx.clone());
                                drop(clients_lock);

                                username = Some(new_username.clone());

                                let response = Message::ReceiveMessage {
                                    sender: "Server".to_string(),
                                    content: format!("Добро пожаловать {}!", new_username),
                                };
                                send_massage(&mut socket, &response).await;
                            }
                            Message::SendMessage { content } => {
                                if let Some(sender) = &username {
                                    println!("Получено сообщение от {}: {}", sender, content);
                                    let message = serde_json::to_string(&Message::ReceiveMessage {
                                        sender: sender.clone(),
                                        content: content.clone(),
                                    })
                                    .unwrap();

                                    if let Err(e) = tx.send(message) {
                                        eprintln!("Ошибка отправки в канал: {}", e);
                                    }
                                }
                            }
                            Message::Leave => {
                                if let Some(sender) = &username {
                                    println!("Клиент {} покидает чат", sender);

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
                            _ => {}
                        }
                    }

                    result = rx.recv() => {
                        match result {
                            Ok(msg) => {
                                if let Err(e) = socket.write_all(msg.as_bytes()).await {
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
    ReceiveMessage { sender: String, content: String }, // Сообщение для клиента
    Leave, // выход пользователя
    ErrorMessage { error: String }, // Ответ об ошибке
}

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

async fn send_massage(socket: &mut tokio::net::TcpStream, message: &Message) {
    let json_message = serde_json::to_string(message).unwrap();
    if let Err(e) = socket.write_all(json_message.as_bytes()).await {
        error!("Ошибка записи {}", e);
    }
}
