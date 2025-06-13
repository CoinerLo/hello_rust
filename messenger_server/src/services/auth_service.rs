use crate::types::{AppResult, DbPool, ServerError};
use bcrypt::verify;
use tracing::error;
use crate::db::user;

pub async fn register_user(pool: &DbPool, username: &str, password: &str) -> AppResult<()> {
    if user::find_user_by_username(pool, username).await?.is_some() {
        return Err(ServerError::UserExists);
    }
    user::register(pool, username, password).await?;
    Ok(())
}

pub async fn authenticate_user(pool: &DbPool, username: &str, password: &str) -> AppResult<bool> {
    let current_user = user::find_user_by_username(pool, username).await?;
    if let Some((_, password_hash)) = current_user {
        let is_valid = verify(password, &password_hash)
            .map_err(|e| {
                error!("Ошибка проверки пароля: {}", e);
                ServerError::BcryptError(e)
            })?;
        return Ok(is_valid);
    }
    Ok(false)
}
