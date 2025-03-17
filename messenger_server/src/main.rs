use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создаем TCP-слушатель на порту 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Сервер запущен на 127.0.0.1:8080");

    // глобальное состояние сервера
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // Принимаем входящее подключение
        let (mut socket, addr) = listener.accept().await?;
        println!("Новое подключение: {}", addr);

        // Клонируем состояние для каждой задачи
        let clients = clients.clone();

        // Обрабатываем подключение в отдельной задаче
        tokio::spawn(async move {
            let mut buffer = [0; 1024]; // Буфер для чтения данных

            loop {
                // Читаем данные из сокета
                let n = match socket.read(&mut buffer).await {
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
                        clients_lock.insert(new_username.clone(), socket.try_clone().unwrap());
                        drop(clients_lock);

                        let response = Message::ReceiveMessage {
                            sender: "Server".to_string(),
                            content: format!("Добро пожаловать {}!", new_username),
                        };
                        send_massage(&mut socket, &response).await;
                    }
                    Message::SendMessage { content } => {
                        println!("Получено сообщениеЖ {}", content);
                        let response = Message::ReceiveMessage {
                            sender: "Echo".to_string(),
                            content: content,
                        };
                        send_massage(&mut socket, &response).await;
                    }
                    _ => {}
                }
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

type Clients = Arc<Mutex<HashMap<String, tokio::net::TcpStream>>>;

async fn send_massage(socket: &mut tokio::net::TcpStream, message: &Message) {
    let json_message = serde_json::to_string(message).unwrap();
    if let Err(e) = socket.write_all(json_message.as_bytes()).await {
        eprintln!("Ошибка записи {}", e);
    }
}
