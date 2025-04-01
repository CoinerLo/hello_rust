use std::env;

use bb8_postgres::PostgresConnectionManager;
use bb8::Pool;
use tokio_postgres::NoTls;
use dotenv::dotenv;
use tracing::{warn, info, error};

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

// подключение
pub async fn create_db_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not sen in .env file")?;
    info!("Подключение к базе данных: {}", database_url);

    
    let manager = PostgresConnectionManager::new_from_stringlike(database_url,NoTls)
        .map_err(|e| {
            error!("Ошибка создания менеджера соединений: {}", e);
            format!("Ошибка создания менеджера соединений: {}", e)
        })?;

    let pool = Pool::builder()
        .build(manager)
        .await
        .map_err(|e| {
            error!("Ошибка создания пула соединений: {}", e);
            format!("Ошибка создания пула соединений: {}", e)
        })?;

    info!("Пул соединений успешно создан");
    Ok(pool)
}

// сохранение сообщения
pub async fn save_message(pool: &DbPool, sender: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Сохранение в базу данных сообщения: sender={}, content={}", sender, content);

    let client = pool
        .get()
        .await
        .map_err(|e| {
            error!("Ошибка получения соединения из пула: {}", e);
            format!("Ошибка получения соединения из пула: {}", e)
        })?;

    let rows_affected = client
        .execute(
            "INSERT INTO messages (sender, content) VALUES ($1, $2)",
            &[&sender, &content],
        )
        .await
        .map_err(|e| {
            error!("Ошибка выполнения запроса INSERT: {}", e);
            format!("Ошибка выполнения запроса INSERT: {}", e)
        })?;

    if rows_affected == 0 {
        warn!("Сообщение не было сохранено в базе данных");
    } else {
        info!("Сообщение было сохранено в базе данных");
    }

    Ok(())
}

// Загрузка истории
pub async fn load_history(pool: &DbPool, limit: i64) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    info!("Загрузка истории сообщений (limit={})", limit);

    let client = pool
        .get()
        .await
        .map_err(|e| {
            error!("Ошибка получения соединения из пула: {}", e);
            format!("Ошибка получения соединения из пула: {}", e)
        })?;

    let rows = client
        .query(
            "SELECT sender, content FROM messages ORDER BY timestamp DESC LIMIT $1",
            &[&limit],
        )
        .await
        .map_err(|e| {
            error!("Ошибка выполнения запроса SELECT: {}", e);
            format!("Ошибка выполнения запроса SELECT: {}", e)
        })?;

    let history: Vec<(String, String)> = rows
        .iter()
        .map(|row| {
            let sender = row.get(0);
            let content = row.get(1);
            (sender, content)
        })
        .collect();

    info!("Загружено {} сообщений из базы данных", history.len());
    Ok(history)
}

pub async fn register_user(
    pool: &DbPool,
    username: &str,
    password: &str
) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}
