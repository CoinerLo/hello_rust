use crate::{db::group_chat, types::{AppResult, DbPool, ServerError}};
use tracing::error;

pub async fn create_group_chat(pool: &DbPool, name: &str, creator: &str) -> AppResult<i32> {
    group_chat::create(pool, name, creator)
        .await
        .map_err(|e| {
            error!("Ошибка создания группового чата: {}", e);
            ServerError::CreateGroupChatError
        })
}
