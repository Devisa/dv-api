pub mod user;
pub mod topic;
pub mod group;

use crate::util::respond;
use ap_com::{Db, Model};
use ap_com::models::group::Group;
use actix_web::{
    http::StatusCode,
    web::{HttpRequest,  ServiceConfig, self}, Responder
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
