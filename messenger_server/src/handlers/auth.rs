use crate::{services::auth_service, types::DbPool};
use actix_web::{web, HttpResponse, Responder};
use tracing::error;
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct RegisterUser {
    username: String,
    password: String,
}

#[derive(serde::Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

pub async fn register(
    pool: web::Data<Arc<DbPool>>,
    form: web::Json<RegisterUser>,
) -> impl Responder {
    match auth_service::register_user(pool.get_ref(), &form.username, &form.password).await {
        Ok(_) => HttpResponse::Ok().body("Пользователь успешно зарегистрирован"),
        Err(e) => {
            error!("Ошибка регистрации пользователя: {}", e);
            HttpResponse::BadRequest().body("Ошибка регистрации")
        },
    }
}

pub async fn login(
    pool: web::Data<Arc<DbPool>>,
    form: web::Json<LoginUser>,
) -> impl Responder {
    match auth_service::authenticate_user(pool.get_ref(), &form.username, &form.password).await {
        Ok(true) => HttpResponse::Ok().body("Пользователь успешно вошел"),
        Ok(false) => HttpResponse::Unauthorized().body("Неверный логин или пароль"),
        Err(e) => {
            error!("Ошибка входа пользователя: {}", e);
            HttpResponse::InternalServerError().body("Ошибка авторизации")
        },
    }
}
