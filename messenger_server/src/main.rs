use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создаем TCP-слушатель на порту 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Сервер запущен на 127.0.0.1:8080");

    loop {
        // Принимаем входящее подключение
        let (mut socket, addr) = listener.accept().await?;
        println!("Новое подключение: {}", addr);

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

                // Выводим полученные данные
                println!(
                    "Получено от клиента {}: {:?}",
                    addr,
                    String::from_utf8_lossy(&buffer[..n])
                );

                // Отправляем эхо-ответ
                if let Err(e) = socket.write_all(&buffer[..n]).await {
                    eprintln!("Ошибка записи клиенту {}: {}", addr, e);
                    return;
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
