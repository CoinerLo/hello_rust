use actix_web::{web, HttpResponse, Responder};
use tracing::error;
use crate::{services::chat_service, types::DbPool};

#[derive(serde::Deserialize)]
pub struct CreateChat {
    name: String,
    creator: String,
}

#[derive(serde::Deserialize)]
pub struct DeleteChat {
    chat_id: i32,
    requester: String,
}

pub async fn create(
    pool: web::Data<DbPool>,
    form: web::Json<CreateChat>,
) -> impl Responder {
    match chat_service::create_group_chat(&pool, &form.name, &form.creator).await {
        Ok(chat_id) => HttpResponse::Ok().body(format!("Чат создан с ID: {}", chat_id)),
        Err(e) => {
            error!("Ошибка создания чата {}", e);
            HttpResponse::BadRequest().body("Ошибка создания чата")
        },
    }
}

pub async fn delete(
    pool: web::Data<DbPool>,
    data: web::Json<DeleteChat>,
) -> impl Responder {
    match chat_service::delete_group_chat(&pool, data.chat_id, &data.requester).await {
        Ok(_) => HttpResponse::Ok().body("Чат удален"),
        Err(e) => {
            error!("Ошибка удаления чата {}", e);
            HttpResponse::BadRequest().body("Ошибка удаления чата")
        },
    }
}
