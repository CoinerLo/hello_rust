use bb8_postgres::PostgresConnectionManager;
use bb8::{Pool, RunError};
use tokio_postgres::{NoTls, Error};
use tracing::{error};

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

// подключение
pub async fn create_db_pool() -> Result<DbPool, RunError<Error>> {
    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost dbname=chat_server user=postgres password=111",
        NoTls,
    )
    .unwrap();

    let pool = Pool::builder().build(manager).await?;

    Ok(pool)
}

// сохранение сообщения
pub async fn save_message(pool: DbPool, sender: &str, content: &str) -> Result<(), RunError<Error>> {
    let client = pool.get().await?;
    client
        .execute(
            "INSERT INTO messages (sender, content) VALUES ($1, $2)",
            &[&sender, &content],
        )
        .await?;
    Ok(())
}

// Загрузка истории
pub async fn load_history(pool: DbPool, limit: i64) -> Result<Vec<(String, String)>, RunError<Error>> {
    let client = pool.get().await?;
    let rows = client
        .query(
            "SELECT sender, content FROM messages ORDER BY timestamp DESC LIMIT $1",
            &[&limit],
        )
        .await?;

    let history: Vec<(String, String)> = rows
        .iter()
        .map(|row| {
            let sender = row.get(0);
            let content = row.get(1);
            (sender, content)
        })
        .collect();
    Ok(history)
}
