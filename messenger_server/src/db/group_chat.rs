use tracing::{warn, info, error};

use crate::{types::{AppResult, DbPool, ServerError}, structs::Chat};


pub async fn get_all(pool: &DbPool) -> AppResult<Vec<Chat>> {
    let chats = sqlx::query_as!(
        Chat,
        "SELECT id, name, creator FROM group_chats",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("Ошибка получения списка чатов из БД: {}", e);
        ServerError::DatabaseError { context: "Ошибка получения списка чатов из БД".to_string(), source: e }
    })?;

    Ok(chats)
}

// создание нового группового чата
pub async fn create(
    pool: &DbPool,
    name: &str,
    creator: &str,
) -> AppResult<i32> {
    info!("Попытка создания группового чата: {} (создатель: {})", name, creator);

    let count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM group_chats WHERE name = $1",
        name
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("Ошибка получения списка чатов из БД: {}", e);
        ServerError::DatabaseError { context: "Ошибка получения списка чатов из БД".to_string(), source: e }
    })?;
    
    let count = count.unwrap_or(0);
    if count > 0 {
        warn!("Групповой чат с именем {} уже существуют", name);
        return Err(ServerError::GroupChatExist);
    }

    let row = sqlx::query!(
        "INSERT INTO group_chats (name, creator) VALUES ($1, $2) RETURNING id",
        name,
        creator,
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("Ошибка записи группового чата в БД: {}", e);
        ServerError::DatabaseError { context: "Ошибка записи группового чата в БД".to_string(), source: e }
    })?;
    let chat_id = row.id;
    info!("групповой чат {} успешно создан (ID: {}, создатель: {})", name, chat_id, creator);
    Ok(chat_id)
}

// Добавление участника в групповой чат
pub async fn add_member(
    pool: &DbPool,
    chat_id: i32,
    username: &str,
) -> AppResult<()> {
    info!("Попытка добавления участника {} в групповой чат ID: {}", username, chat_id);

    sqlx::query!(
        "INSERT INTO group_chat_members (chat_id, username) VALUES ($1, $2)",
        chat_id,
        username,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Ошибка добавления пользователя в групповой чат (chat_id={}) в БД: {}", chat_id, e);
        ServerError::DatabaseError { context: "Ошибка добавления пользователя в групповой чат".to_string(), source: e }
    })?;

    info!("Участник {} успешно добавлен в групповой чат ID: {}", username, chat_id);
    Ok(())
}

// получение списка участников группового чата
pub async fn get_members(
    pool: &DbPool,
    chat_id: i32,
) -> AppResult<Vec<String>> {
    info!("Попытка получения списка участников группового чата ID: {}", chat_id);

    let rows = sqlx::query!(
        "SELECT username FROM group_chat_members WHERE chat_id = $1", 
        chat_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("Ошибка добавления пользователя в групповой чат (chat_id={}) в БД: {}", chat_id, e);
        ServerError::DatabaseError { context: "Ошибка добавления пользователя в групповой чат".to_string(), source: e }
    })?;

    let members: Vec<String> = rows.into_iter().map(|row| row.username).collect();
    info!("Найдено {} участников в групповом чате ID: {}", members.len(), chat_id);

    Ok(members)
}

async fn check_if_creator(
    pool: &DbPool,
    chat_id: i32,
    username: &str
) -> AppResult<bool> {
    let row = sqlx::query!(
            "SELECT creator FROM group_chats WHERE id = $1",
            chat_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("Ошибка получения создателя чата из БД: {}", e);
            ServerError::DatabaseError { context: "Ошибка получения создателя чата из БД".to_string(), source: e }
        })?;

    Ok(row.creator == username)
}

// удаление участника из группового чата
pub async fn remove_member(
    pool: &DbPool,
    chat_id: i32,
    username: &str,
    requester: &str,
) -> AppResult<()> {
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

    let rows_affected = sqlx::query!(
        "DELETE FROM group_chat_members WHERE chat_id = $1 AND username = $2", 
        chat_id,
        username,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Ошибка (БД) удаления из группового чата: {}", e);
        ServerError::DatabaseError { context: "Ошибка (БД) удаления из группового чата".to_string(), source: e }
    })?
    .rows_affected();

    if rows_affected == 0 {
        warn!("Пользователь {} не является создателем чата ID: {}", requester, chat_id);
        return Err(ServerError::MemberNotFound);
    }

    info!("Участник {} успешно удален из группового чата ID: {}", username, chat_id);
    Ok(())
}

// удаление группового чата
pub async fn delete(
    pool: &DbPool,
    chat_id: i32,
    requester: &str,
) -> AppResult<()> {
    info!("Попытка удаления группвого чата ID: {} (запросил: {})", chat_id, requester);

    let is_creator = check_if_creator(pool, chat_id, requester).await?;
    if !is_creator {
        warn!("Пользователь {} не является создателем чата ID: {}", requester, chat_id);
        return Err(ServerError::PermissionDenied);
    }

    sqlx::query!(
        "DELETE FROM group_chats WHERE id = $1", 
        chat_id,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Ошибка (БД) удаления группового чата: {}", e);
        ServerError::DatabaseError { context: "Ошибка (БД) удаления группового чата".to_string(), source: e }
    })?;

    info!("Групповой чат (ID = {}) успешно удален", chat_id);
    Ok(())
}
