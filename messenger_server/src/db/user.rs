use std::env::{self, VarError};
use bb8_postgres::PostgresConnectionManager;
use bb8::{Pool, RunError};
use bcrypt::{hash, verify, DEFAULT_COST};
use tokio_postgres::NoTls;
use dotenv::dotenv;
use tracing::{warn, info, error};

use crate::types::ServerError;


pub async fn register(
    pool: &DbPool,
    username: &str,
    password: &str
) -> AppResult<()> {
    let client = pool
        .get()
        .await
        .map_err(|e| {
            error!("Ошибка получения соединения из пула: {}", e);
            ServerError::DatabaseError(e.into())
        })?;

    let password_hash = hash(password, DEFAULT_COST)
        .map_err(|e| {
            error!("Ошибка хэширования пароля: {}", e);
            ServerError::BcryptError(e.into())
        })?;

    // Проверяем, что пользователь с таким именем не существует
    let rows = client
        .query("SELECT COUNT(*) FROM users WHERE username = $1", &[&username])
        .await
        .map_err(|e| {
            error!("Ошибка выполнения запроса SELECT для записи пользователя: {}", e);
            ServerError::DatabaseError(e.into())
        })?;

    let count: i64 = rows[0].get(0);
    if count > 0 {
        warn!("Пользователь с именем {} уже существует", username);
        return Err(ServerError::UserExists);
    }

    // записываем пользователя в базу
    client
        .execute(
            "INSERT INTO users (username, password_hash) VALUES ($1, $1)",
            &[&username, &password_hash],
        )
        .await
        .map_err(|e| {
            error!("Ошибка выполнения запроса записи в базу пользователя: {}", e);
            ServerError::DatabaseError(e.into())
        })?;

    info!("Пользователь {} успешно зарегистрирован", username);
    Ok(())
}
