use thiserror::Error;
use std::env::VarError;
use sqlx::{PgPool, Error as SqlxError};
use bcrypt::BcryptError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Ошибка базы данных ({context}): {source}")]
    DatabaseError {
        context: String,
        #[source]
        source: SqlxError,
    },
    #[error("Ошибка хэширования пароля: {0}")]
    BcryptError(#[from] BcryptError),
    #[error("Ошибка отправки сообщения: {0}")]
    MessageSendError(#[from] std::io::Error),
    #[error("Пользователь с таким именем уже существует")]
    UserExists,
    #[error("DATABASE_URL is not sen in .env file")]
    VarError(#[from] VarError),
    #[error("Групповой чат с таким именем уже существует")]
    GroupChatExist,
    #[error("Для выполнения действия не хватает прав")]
    PermissionDenied,
    #[error("Пользователь не найден")]
    MemberNotFound,
    #[error("Недопустимая операция")]
    InvalidOperation,
    #[error("Ошибка создания группового чата")]
    CreateGroupChatError,
    #[error("Ошибка удаления группового чата")]
    DeleteGroupChatError,
    #[error("Ошибка загрузки списка чатов")]
    GetAllGroupChatsError,
}

pub type AppResult<T> = Result<T, ServerError>;

pub type DbPool = PgPool;
