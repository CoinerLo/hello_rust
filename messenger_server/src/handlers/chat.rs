use actix_web::{web, HttpResponse, Responder};
use tracing::error;
use crate::{services::chat_service, types::DbPool};



#[derive(serde::Deserialize)]
struct CreateChat {
    name: String,
    creator: String,
}

pub async fn crrate_chat(
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
