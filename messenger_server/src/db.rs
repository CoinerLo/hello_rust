use tokio_postgres::{NoTls, Error};
use tracing::{error};

// подключение
pub async fn connecto_to_db() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect("host=localhost dbname=chat_server user=postgres password=111", NoTls).await?;

    // запустить задачу для обработки соединения
    tokio::spawn(async move {
        if let Err(e) = connection.await{
            error!("Ошибка подключения к базе данных: {}", e);
        }
    });

    Ok(client)
}

// сохранение сообщения
pub async fn save_message(client: &tokio_postgres::Client, sender: &str, content: &str) -> Result<(), Error> {
    client
        .execute(
            "INSERT INTO messages (sender, content) VALUES ($1, $2)",
            &[&sender, &content],
        )
        .await?;
    Ok(())
}
