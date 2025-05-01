use std::env::{self};
use sqlx::PgPool;
use dotenv::dotenv;
use tracing::{info, error};
use crate::types::{AppResult, ServerError};

// подключение
pub async fn create_db_pool() -> AppResult<PgPool> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|e| {
            error!("DATABASE_URL is not sen in .env file: {}", e);
            ServerError::VarError(e.into())
        })?;
    info!("Подключение к базе данных: {}", database_url);


    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|e| {
            error!("Ошибка создания пула соединений: {}", e);
            ServerError::PoolError(e)
        })?;

    info!("Пул соединений успешно создан");
    Ok(pool)
}
