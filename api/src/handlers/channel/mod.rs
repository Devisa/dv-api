pub mod topic;
pub mod group;

use api_db::{Model, Id, Db};
use crate::util::respond;
use api_common::models::{channel::Channel, group::Group};
use sqlx::{prelude::*, postgres::Postgres};
use actix_web::{
    Responder,
    web::{Json, Path, Data, HttpRequest,  ServiceConfig, self},
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_channel))
        )
        .service(web::resource("/msg")
            .route(web::get().to(get_all_messages))
            .route(web::post().to(new_message))
        )
        .service(web::scope("/{channel_id}")
            .route("/start", web::get().to(start_channel))
            .route("/stop", web::get().to(stop_channel))
            .service(web::resource("")
                .route(web::get().to(get_by_id))
                .route(web::delete().to(delete_by_id))
            )
        )
        .service(web::scope("/id/{message_id}")
            .route("", web::get().to(get_message_by_id))
        )
        .service(web::scope("/group").configure(group::routes))
        .service(web::scope("/topic").configure(topic::routes));

}

// #[get("/")]
pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Channel::get_all(&db.pool).await {
        Ok(ch) => respond::ok(ch),
        Err(e) => respond::err(e)
    }
}
// #[get("/msg")]
pub async fn get_all_messages(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn new_message(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn new_channel(db: Data<Db>, ch: Json<Channel>) -> impl Responder {
    match ch.into_inner().insert(&db.pool).await {
        Ok(ch) => respond::ok(ch),
        Err(e) => respond::err(e)
    }
}
// #[get("/{channel_id}")]
pub async fn get_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Channel::get(&db.pool, id.into_inner()).await {
        Ok(Some(ch)) => respond::ok(ch),
        Ok(None) => respond::not_found("No channel with that id"),
        Err(e) => respond::err(e)
    }
}

// #[get("/{channel_id}/start")]
pub async fn start_channel(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Channel::get(&db.pool, id.into_inner()).await {
        Ok(Some(ch)) => {
            respond::ok(ch)
        }
        Ok(None) => respond::not_found("No channel with that id"),
        Err(e) => respond::err(e)
    }
}
pub async fn stop_channel(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Channel::get(&db.pool, id.into_inner()).await {
        Ok(Some(ch)) => {
            respond::ok(ch)
        }
        Ok(None) => respond::not_found("No channel with that id"),
        Err(e) => respond::err(e)
    }
}
pub async fn delete_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Channel::delete(&db.pool, id.into_inner()).await {
        Ok(Some(ch)) => respond::ok(ch),
        Ok(None) => respond::not_found("No channel with that id"),
        Err(e) => respond::err(e)
    }
}
pub async fn delete_all(db: Data<Db>) -> impl Responder {
    match Channel::delete_all(&db.pool).await {
        Ok(ch) => respond::ok(ch),
        Err(e) => respond::err(e)
    }
}
pub async fn get_message_by_id(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
