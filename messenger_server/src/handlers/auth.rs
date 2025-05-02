use crate::{services::auth_service, types::DbPool};
use actix_web::{web, HttpResponse, Responder};
use tracing::error;

#[derive(serde::Deserialize)]
struct RegisterUser {
    username: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

pub async fn register(
    pool: web::Data<DbPool>,
    form: web::Json<RegisterUser>,
) -> impl Responder {
    match auth_service::register_user(&pool, &form.username, &form.password).await {
        Ok(_) => HttpResponse::Ok().body("Пользователь успешно зарегистрирован"),
        Err(e) => {
            error!("Ошибка регистрации пользователя: {}", e);
            HttpResponse::BadRequest().body(format!("Ошибка регистрации"))
        },
    }
}
