use bcrypt::{hash, verify, DEFAULT_COST};
use tracing::{warn, info, error};

use crate::types::{AppResult, DbPool, ServerError};

pub async  fn find_user_by_username(pool: &DbPool, username: &str,) -> AppResult<Option<(String, String)>> {
    let row = sqlx::query!(
        "SELECT username, password_hash FROM users WHERE username = $1",
        username,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("Ошибка при поиске пользователя: {}", e);
        ServerError::DatabaseError { context: "Ошибка при поиске пользователя".to_string(), source: e }
    })?;
    Ok(row.map(|r| (r.username, r.password_hash)))
}

pub async fn register(
    pool: &DbPool,
    username: &str,
    password: &str
) -> AppResult<()> {
    let password_hash = hash(password, DEFAULT_COST)
        .map_err(|e| {
            error!("Ошибка хэширования пароля: {}", e);
            ServerError::BcryptError(e.into())
        })?;

    // // Проверяем, что пользователь с таким именем не существует
    // let rows = sqlx::query!(
    //     "SELECT COUNT(*) FROM users WHERE username = $1",
    //     username
    // )
    // .fetch_all(pool)
    // .await
    // .map_err(|e| {
    //     error!("Ошибка поиска пользователя: {}", e);
    //     ServerError::DatabaseError { context: "Ошибка поиска пользователя".to_string(), source: e }
    // })?
    // .;

    // let count: i64 = rows[0].get(0);
    // if count > 0 {
    //     warn!("Пользователь с именем {} уже существует", username);
    //     return Err(ServerError::UserExists);
    // }

    sqlx::query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        username,
        password_hash,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Ошибка выполнения запроса записи в базу пользователя: {}", e);
        ServerError::DatabaseError { context: "".to_string(), source: e }
    })?;

    info!("Пользователь {} успешно зарегистрирован", username);
    Ok(())
}

// pub async fn authenticate(
//     pool: &DbPool,
//     username: &str,
//     password: &str,
// ) -> AppResult<bool> {
//     let client = pool
//         .get()
//         .await
//         .map_err(|e| {
//             error!("Ошибка получения соединения из пула: {}", e);
//             ServerError::DatabaseError(e.into())
//         })?;

//     // Получаем хеш пароля и базы
//     let rows = client
//         .query("SELECT password_hash FROM users WHERE username = $1", &[&username])
//         .await
//         .map_err(|e| {
//             error!("Ошибка поиска пользователя в базе данных: {}", e);
//             ServerError::DatabaseError { context: "Ошибка поиска пользователя в базе данных".to_string(), source: e }
//         })?;

//     if rows.is_empty() {
//         warn!("Пользователь {} не найден", username);
//         return Ok(false);
//     }

//     let password_hash: String = rows[0].get(0);
//     let is_valid = verify(password, &password_hash)
//         .map_err(|e| {
//             error!("Ошибка проверки пароля: {}", e);
//             ServerError::BcryptError(e)
//         })?;
//     if !is_valid {
//         warn!("Неверный пароль для пользователя {}", username);
//         return Ok(false);
//     }

//     info!("Пользователь {} успешно авторизован", username);
//     Ok(true)
// }
