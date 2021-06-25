pub mod user;
pub mod topic;
pub mod group;

use crate::{db::Db, util::respond};
use api_common::models::group::Group;
use sqlx::{prelude::*, postgres::Postgres};
use actix_web::{
    http::StatusCode,
    delete, get, post, web::{HttpRequest,  ServiceConfig, self}, Responder
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_message))
        )
        .service(web::scope("/id/{message_id}")
            .route("", web::get().to(get_by_id))
        )
        .service(web::scope("/group").configure(group::routes))
        .service(web::scope("/user").configure(user::routes))
        .service(web::scope("/topic").configure(topic::routes));
}



pub async fn get_all(db: web::Data<Db>) -> impl Responder {
    "GET /message".to_string()
}
pub async fn new_message(db: web::Data<Db>, group: web::Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn get_by_id(db: web::Data<Db>, group: web::Json<Group>) -> impl Responder {
    "".to_string()
}
