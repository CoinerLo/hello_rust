use std::env::{self, VarError};
use thiserror::Error;
use bb8_postgres::PostgresConnectionManager;
use bb8::{Pool, RunError};
use bcrypt::{hash, verify, DEFAULT_COST};
use tokio_postgres::NoTls;
use dotenv::dotenv;
use tracing::{warn, info, error};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Ошибка базы данных: {0}")]
    DatabaseError(#[from] RunError<tokio_postgres::Error>),
    #[error("Ошибка хэширования пароля: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("Ошибка отправки сообщения: {0}")]
    MessageSendError(#[from] std::io::Error),
    #[error("Пользователь с таким именем уже существует")]
    UserExists,
    #[error("Ошибка создания менеджера соединений")]
    CreateManagerError,
    #[error("DATABASE_URL is not sen in .env file")]
    VarError(#[from] VarError),
    #[error("Ошибка создания пула соединений")]
    PoolError,
    #[error("Групповой чат с таким именем уже существует")]
    GroupChatExist,
    #[error("Для выполнения действия не хватает прав")]
    PermissionDenied,
    #[error("Пользователь не найден")]
    MemberNotFound,
    #[error("Недопустимая операция")]
    InvalidOperation,
}

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

pub type AppResult<T> = Result<T, ServerError>;

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

// авторизация пользователя
pub async fn authenticate_user(
    pool: &DbPool,
    username: &str,
    password: &str,
) -> AppResult<bool> {
    let client = pool
        .get()
        .await
        .map_err(|e| {
            error!("Ошибка получения соединения из пула: {}", e);
            ServerError::DatabaseError(e.into())
        })?;

    // Получаем хеш пароля и базы
    let rows = client
        .query("SELECT password_hash FROM users WHERE username = $1", &[&username])
        .await
        .map_err(|e| {
            error!("Ошибка поиска пользователя в базе данных: {}", e);
            ServerError::DatabaseError(e.into())
        })?;

    if rows.is_empty() {
        warn!("Пользователь {} не найден", username);
        return Ok(false);
    }

    let password_hash: String = rows[0].get(0);
    let is_valid = verify(password, &password_hash)
        .map_err(|e| {
            error!("Ошибка проверки пароля: {}", e);
            ServerError::BcryptError(e)
        })?;
    if !is_valid {
        warn!("Неверный пароль для пользователя {}", username);
        return Ok(false);
    }

    info!("Пользователь {} успешно авторизован", username);
    Ok(true)
}

// создание нового группового чата
pub async fn create_group_chat(
    pool: &DbPool,
    name: &str,
    creator: &str,
) -> AppResult<i32> {
    let client = pool
        .get()
        .await
        .map_err(|e| {
            error!("Ошибка получения соединения из пула: {}", e);
            ServerError::DatabaseError(e.into())
        })?;
    info!("Попытка создания группового чата: {} (создатель: {})", name, creator);

    let rows = client
        .query(
    "SELECT COUNT(*) FROM group_chats WHERE name = $1",
        &[&name])
        .await
        .map_err(|e| {
            error!("Ошибка получения списка чатов из БД: {}", e);
            ServerError::DatabaseError(e.into())
        })?;
    
    let count: i64 = rows[0].get(0);
    if count > 0 {
        warn!("Групповой чат с именем {} уже существуют", name);
        return Err(ServerError::GroupChatExist);
    }

    let row = client
        .query_one(
            "INSERT INTO group_chats (name, creator) VALUES ($1, &2) RETURNING id",
            &[&name, &creator],
        )
        .await
        .map_err(|e| {
            error!("Ошибка записи группового чата в БД: {}", e);
            ServerError::DatabaseError(e.into())
        })?;
    let chat_id = row.get(0);
    info!("групповой чат {} успешно создан (ID: {}, создатель: {})", name, chat_id, creator);
    Ok(chat_id)
}

// Добавление участника в групповой чат
pub async fn add_member_to_group_chat(
    pool: &DbPool,
    chat_id: i32,
    username: &str,
) -> AppResult<()> {
    let client = pool
        .get()
        .await
        .map_err(|e| {
            error!("Ошибка получения соединения из пула: {}", e);
            ServerError::DatabaseError(e.into())
        })?;
    info!("Попытка добавления участника {} в групповой чат ID: {}", username, chat_id);

    client
        .execute(
            "INSERT INTO group_chat_members (chat_id, username) VALUES ($1, $2)",
            &[&chat_id, &username],
        )
        .await
        .map_err(|e| {
            error!("Ошибка добавления пользователя в групповой чат (chat_id={}) в БД: {}", chat_id, e);
            ServerError::DatabaseError(e.into())
        })?;

    info!("Участник {} успешно добавлен в групповой чат ID: {}", username, chat_id);
    Ok(())
}

// получение списка участников группового чата
pub async fn get_group_chat_members(
    pool: &DbPool,
    chat_id: i32,
) -> AppResult<Vec<String>> {
    let client = pool
        .get()
        .await
        .map_err(|e| {
            error!("Ошибка получения соединения из пула: {}", e);
            ServerError::DatabaseError(e.into())
        })?;
    info!("Попытка получения списка участников группового чата ID: {}", chat_id);

    let rows = client
        .query(
            "SELECT username FROM group_chat_members WHERE chat_id = $1", 
            &[&chat_id],
        )
        .await
        .map_err(|e| {
            error!("Ошибка добавления пользователя в групповой чат (chat_id={}) в БД: {}", chat_id, e);
            ServerError::DatabaseError(e.into())
        })?;

    let members: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
    info!("Найдено {} участников в групповом чате ID: {}", members.len(), chat_id);

    Ok(members)
}

async fn check_if_creator(
    pool: &DbPool,
    chat_id: i32,
    username: &str
) -> AppResult<bool> {
    let client = pool
    .get()
    .await
    .map_err(|e| {
        error!("Ошибка получения соединения из пула: {}", e);
        ServerError::DatabaseError(e.into())
    })?;
    let row = client
        .query_one(
            "SELECT creator FROM group_chats WHERE id = $1",
            &[&chat_id]
        )
        .await
        .map_err(|e| {
            error!("Ошибка получения создателя чата из БД: {}", e);
            ServerError::DatabaseError(e.into())
        })?;

    let creator: String = row.get(0);
    Ok(creator == username)
}

// удаление участника из группового чата
pub async fn remove_member_from_group_chat(
    pool: &DbPool,
    chat_id: i32,
    username: &str,
    requester: &str,
) -> AppResult<()> {
    let client = pool
    .get()
    .await
    .map_err(|e| {
        error!("Ошибка получения соединения из пула: {}", e);
        ServerError::DatabaseError(e.into())
    })?;
    info!("Попытка удаления участника {} из группового чата ID: {} (запросил: {})", username, chat_id, requester);

    let is_creator = check_if_creator(pool, chat_id, requester).await?;
    if !is_creator {
        warn!("Пользователь {} не является создателем чата ID: {}", requester, chat_id);
        return Err(ServerError::PermissionDenied);
    }

    if requester == username {
        warn!("Создатель чата не может удалить самого себя");
        return Err(ServerError::InvalidOperation);
    }

    let rows_affected = client
        .execute(
            "DELETE FROM group_chat_members WHERE chat_id = $1 AND username = $2", 
            &[&chat_id, &username],
        )
        .await
        .map_err(|e| {
            error!("Ошибка (БД) удаления из группового чата: {}", e);
            ServerError::DatabaseError(e.into())
        })?;
    
    if rows_affected == 0 {
        warn!("Пользователь {} не является создателем чата ID: {}", requester, chat_id);
        return Err(ServerError::MemberNotFound);
    }

    info!("Участник {} успешно удален из группового чата ID: {}", username, chat_id);
    Ok(())
}

// удаление группового чата
pub async fn delete_group_chat(
    pool: &DbPool,
    chat_id: i32,
    requester: &str,
) -> AppResult<()> {
    let client = pool
    .get()
    .await
    .map_err(|e| {
        error!("Ошибка получения соединения из пула: {}", e);
        ServerError::DatabaseError(e.into())
    })?;
    info!("Попытка удаления группвого чата ID: {} (запросил: {})", chat_id, requester);

    let is_creator = check_if_creator(pool, chat_id, requester).await?;
    if !is_creator {
        warn!("Пользователь {} не является создателем чата ID: {}", requester, chat_id);
        return Err(ServerError::PermissionDenied);
    }

    client
        .execute(
            "DELETE FROM group_chats WHERE id = $1", 
            &[&chat_id],
        )
        .await
        .map_err(|e| {
            error!("Ошибка (БД) удаления группового чата: {}", e);
            ServerError::DatabaseError(e.into())
        })?;

    info!("Групповой чат (ID = {}) успешно удален", chat_id);
    Ok(())
}
