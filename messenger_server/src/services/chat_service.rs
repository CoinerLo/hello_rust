use crate::{db::group_chat, types::{AppResult, DbPool, ServerError}, structs::Chat};
use tracing::error;

pub async fn create_group_chat(pool: &DbPool, name: &str, creator: &str) -> AppResult<i32> {
    group_chat::create(pool, name, creator)
        .await
        .map_err(|e| {
            error!("Ошибка создания группового чата: {}", e);
            ServerError::CreateGroupChatError
        })
}

pub async fn delete_group_chat(pool: &DbPool, chat_id: i32, requester: &str) -> AppResult<()> {
    group_chat::delete(pool, chat_id, requester)
        .await
        .map_err(|e| {
            error!("Ошибка удаления группового чата: {}", e);
            ServerError::DeleteGroupChatError
        })
}

pub async fn get_all_group_chats(pool: &DbPool) -> AppResult<Vec<Chat>> {
    group_chat::get_all(pool)
        .await
        .map_err(|e| {
            error!("Ошибка загрузки списка чатов: {}", e);
            ServerError::GetAllGroupChatsError
        })
}
