use tracing::{warn, info, error};

use crate::types::{AppResult, DbPool, ServerError};

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
        ServerError::DatabaseError(e.into())
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
        ServerError::DatabaseError(e.into())
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
pub async fn get_members(
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
pub async fn remove_member(
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
pub async fn delete(
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
