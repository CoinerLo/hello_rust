use std::env::{self};
use bb8_postgres::PostgresConnectionManager;
use bb8::Pool;
use tokio_postgres::NoTls;
use dotenv::dotenv;
use tracing::{info, error};
use crate::types::{AppResult, ServerError};

// подключение
pub async fn create_db_pool() -> AppResult<Pool<PostgresConnectionManager<NoTls>>> {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
      .map_err(|e| {
          error!("DATABASE_URL is not sen in .env file: {}", e);
          ServerError::VarError(e.into())
      })?;
  info!("Подключение к базе данных: {}", database_url);

  
  let manager = PostgresConnectionManager::new_from_stringlike(database_url,NoTls)
      .map_err(|e| {
          error!("Ошибка создания менеджера соединений: {}", e);
          ServerError::CreateManagerError
      })?;

  let pool = Pool::builder()
      .build(manager)
      .await
      .map_err(|e| {
          error!("Ошибка создания пула соединений: {}", e);
          ServerError::PoolError
      })?;

  info!("Пул соединений успешно создан");
  Ok(pool)
}
