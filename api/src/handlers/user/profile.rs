use api_db::{Model, Id, Db};
use crate::util::respond;
use api_common::models::{Profile, verification::VerificationRequest};
use sqlx::{prelude::*, postgres::Postgres};
use actix_web::{
    delete, HttpRequest, HttpResponse, Responder, get, http::StatusCode, post,
    web::{self, Path, Data, Json, ServiceConfig}};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(get_all)
        .service(get_by_id)
        .service(delete_by_id)
        .service(get_by_user_id);
}

#[get("/")]
pub async fn get_all(req: HttpRequest, db: Data<Db>) -> impl Responder {
    match Profile::get_all(&db.pool).await {
        Ok(ver) => respond::found(ver),
        Err(e) => respond::err(e),
    }
}

#[get("/{profile_id}")]
pub async fn get_by_id(req: HttpRequest, db: Data<Db>, profile_id: Path<Id>) -> impl Responder {
    match Profile::get(&db.pool, profile_id.into_inner()).await {
        Ok(ver) => respond::found(ver),
        Err(e) => respond::err(e),
    }
}
#[delete("/{profile_id}")]
pub async fn delete_by_id(req: HttpRequest, db: Data<Db>, profile_id: Path<Id>) -> impl Responder {
    match Profile::delete(&db.pool, profile_id.into_inner()).await {
        Ok(ver) => respond::ok(ver),
        Err(e) => respond::err(e),
    }
}
#[get("/user/{user_id}")]
pub async fn get_by_user_id(req: HttpRequest, db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Profile::get_all_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(ver) => respond::found(ver),
        Err(e) => respond::err(e),
    }

}
