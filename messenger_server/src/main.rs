use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Сервер запущен на 127.0.0.1:8080");

    loop {
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Ошибка чтения: {}", e);
                return;
            }
        };
        println!("ПолученоЖ {:?}", &buffer[..n]);
        if let Err(e) = socket.write_all(&buffer[..n]).await {
            eprintln!("Ошибка записи: {}", e);
            return;
        }
    }
}
