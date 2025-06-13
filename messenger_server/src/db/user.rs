use bcrypt::{hash, DEFAULT_COST};
use tracing::{info, error};

use crate::types::{AppResult, DbPool, ServerError};

pub async fn find_user_by_username(pool: &DbPool, username: &str,) -> AppResult<Option<(String, String)>> {
    let row = sqlx::query!(
        "SELECT username, password_hash FROM users WHERE username = $1",
        username,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("Ошибка при поиске пользователя: {}", e);
        ServerError::DatabaseError { context: "Ошибка при поиске пользователя".to_string(), source: e }
    })?;
    Ok(row.map(|r| (r.username, r.password_hash)))
}

pub async fn register(
    pool: &DbPool,
    username: &str,
    password: &str
) -> AppResult<()> {
    let password_hash = hash(password, DEFAULT_COST)
        .map_err(|e| {
            error!("Ошибка хэширования пароля: {}", e);
            ServerError::BcryptError(e.into())
        })?;

    sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        username,
        password_hash,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Ошибка выполнения запроса записи в базу пользователя: {}", e);
        ServerError::DatabaseError { context: "".to_string(), source: e }
    })?;

    info!("Пользователь {} успешно зарегистрирован", username);
    Ok(())
}
