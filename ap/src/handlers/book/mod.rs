use crate::{db::Db, util::respond, auth::jwt};
use ap_com::models::{post::Post, Session, user::{credentials::Credentials, verification::VerificationRequest}};
use sqlx::{prelude::*, postgres::Postgres};
use actix_web::{Responder, HttpRequest, HttpResponse, HttpResponseBuilder, get, http::StatusCode, post, web::{self, ServiceConfig}};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_book))
        )
        .service(web::scope("/group")
            .service(web::scope("/{group_id}")
                .route("", web::get().to(get_all_books_in_group))
                .route("", web::post().to(new_book_in_group_inbox))
            )
        )
        .service(web::scope("/record")
            .service(web::scope("/{record_id}")
                .route("", web::get().to(get_all_books_in_record))
                .route("", web::post().to(new_book_in_record_inbox))
            )
        )
        .service(web::scope("/topic")
            .service(web::scope("/{topic_id}")
                .route("", web::get().to(get_all_books_in_topic))
                .route("", web::post().to(new_book_in_topic_inbox))
            )
        )
        .service(web::scope("/{book_id}")
            .service(web::resource("")
                .route(web::get().to(get_by_id))
                .route(web::post().to(update_by_id))
                .route(web::delete().to(delete_by_id))
            )
        )
        .service(web::scope("/user")
            .service(web::scope("/{user_id}")
                .route("", web::get().to(get_all_books_by_user))
                .route("", web::post().to(new_book_by_user))
            )
        );
}

pub async fn get_all() -> impl Responder {
    "".to_string()
}
pub async fn new_book() -> impl Responder {
    "".to_string()
}
pub async fn get_all_books_by_user() -> impl Responder {
    "".to_string()
}
pub async fn new_book_by_user() -> impl Responder {
    "".to_string()
}
pub async fn get_all_books_in_topic() -> impl Responder {
    "".to_string()
}
pub async fn get_all_books_in_record() -> impl Responder {
    "".to_string()
}
pub async fn get_all_books_in_group() -> impl Responder {
    "".to_string()
}
pub async fn new_book_in_topic_inbox() -> impl Responder {
    "".to_string()
}
pub async fn new_book_in_record_inbox() -> impl Responder {
    "".to_string()
}
pub async fn new_book_in_group_inbox() -> impl Responder {
    "".to_string()
}
pub async fn get_by_id() -> impl Responder {
    "".to_string()
}
pub async fn update_by_id() -> impl Responder {
    "".to_string()
}
pub async fn delete_by_id() -> impl Responder {
    "".to_string()
}
