use std::collections::HashMap;
use std::sync::Arc;
use serde_json;
use tokio::sync::{broadcast, Mutex};

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создаем TCP-слушатель на порту 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Сервер запущен на 127.0.0.1:8080");

    // создаем канал для рассылки сообщений
    let (tx, _) = broadcast::channel(32);

    // глобальное состояние сервера
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // Принимаем входящее подключение
        let (mut socket, addr) = listener.accept().await?;
        println!("Новое подключение: {}", addr);

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
                                println!("Клиент {} присоединился", new_username);

                                // добавляем клиента в список
                                let mut clients_lock = clients.lock().await;
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
                            _ => {}
                        }
                    }

                    result = rx.recv() => {
                        match result {
                            Ok(msg) => {
                                if let Err(e) = socket.write_all(msg.as_bytes()).await {
                                    eprintln!("Ошибка записи: {}", e);
                                    break;
                                } 
                            }
                            Err(e) => {
                                eprintln!("Ошибка получения из каналаЖ {}", e);
                                break;
                            }
                        }
                    }
                }
            }

            if let Some(username) = username {
                let mut clients_lock = clients.lock().await;
                clients_lock.remove(&username);
                println!("Клиент {} отключился", username);
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
}

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

async fn send_massage(socket: &mut tokio::net::TcpStream, message: &Message) {
    let json_message = serde_json::to_string(message).unwrap();
    if let Err(e) = socket.write_all(json_message.as_bytes()).await {
        eprintln!("Ошибка записи {}", e);
    }
}
