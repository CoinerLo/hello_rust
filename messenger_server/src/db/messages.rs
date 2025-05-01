use tracing::{warn, info, error};

use crate::types::{AppResult, DbPool, ServerError};

// сохранение сообщения
pub async fn save_message(pool: &DbPool, sender: &str, content: &str) -> AppResult<()> {
    info!("Сохранение в базу данных сообщения: sender={}, content={}", sender, content);

    let rows_affected = sqlx::query!(
        "INSERT INTO messages (sender, content) VALUES ($1, $2)",
        sender,
        content,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Ошибка записи сообщения в БД: {}", e);
        ServerError::DatabaseError(e.into())
    })?
    .rows_affected();

    if rows_affected == 0 {
        warn!("Сообщение не было сохранено в базе данных");
    } else {
        info!("Сообщение было сохранено в базе данных");
    }

    Ok(())
}

// Загрузка истории
pub async fn load_history(pool: &DbPool, limit: i64) -> AppResult<Vec<(String, String)>> {
    info!("Загрузка истории сообщений (limit={})", limit);

    let rows = sqlx::query!(
        "SELECT sender, content FROM messages ORDER BY timestamp DESC LIMIT $1",
        limit,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("Ошибка получения сообщений из БД: {}", e);
        ServerError::DatabaseError(e.into())
    })?;

    let history: Vec<(String, String)> = rows
        .into_iter()
        .map(|row| {
            let sender = row.sender;
            let content = row.content;
            (sender, content)
        })
        .collect();

    info!("Загружено {} сообщений из базы данных", history.len());
    Ok(history)
}
