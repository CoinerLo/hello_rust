use actix_web::{web, HttpResponse, Responder};
use tracing::error;
use crate::{services::chat_service, types::DbPool};
use std::sync::Arc;

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
    pool: web::Data<Arc<DbPool>>,
    form: web::Json<CreateChat>,
) -> impl Responder {
    match chat_service::create_group_chat(pool.get_ref(), &form.name, &form.creator).await {
        Ok(chat_id) => HttpResponse::Ok().body(format!("Чат создан с ID: {}", chat_id)),
        Err(e) => {
            error!("Ошибка создания чата {}", e);
            HttpResponse::BadRequest().body("Ошибка создания чата")
        },
    }
}

pub async fn delete(
    pool: web::Data<Arc<DbPool>>,
    data: web::Json<DeleteChat>,
) -> impl Responder {
    match chat_service::delete_group_chat(pool.get_ref(), data.chat_id, &data.requester).await {
        Ok(_) => HttpResponse::Ok().body("Чат удален"),
        Err(e) => {
            error!("Ошибка удаления чата {}", e);
            HttpResponse::BadRequest().body("Ошибка удаления чата")
        },
    }
}

pub async fn get_all(pool: web::Data<Arc<DbPool>>) -> impl Responder {
    match chat_service::get_all_group_chats(pool.get_ref()).await {
        Ok(chats) => HttpResponse::Ok().json(chats),
        Err(e) => {
            error!("Ошибка загрузки списка чатов {}", e);
            HttpResponse::BadRequest().body("Ошибка загрузки списка чатов")
        }
    }
}
