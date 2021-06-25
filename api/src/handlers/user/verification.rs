use crate::{util::respond, db::Db};
use api_common::models::{Model, verification::VerificationRequest};
use sqlx::{prelude::*, postgres::Postgres};
use uuid::Uuid;
use actix_web::{HttpRequest, HttpResponse, Responder, get, http::StatusCode, post, web::{self, ServiceConfig, Path, Json, Data}};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
        )
        .service(web::resource("/{user_id}")
            .route(web::get().to(get_by_user_id))
            .route(web::delete().to(delete_by_user_id))
        );
}

pub async fn get_all(db: Data<Db>) -> impl Responder {
    match VerificationRequest::get_all(&db.pool).await {
        Ok(ver) => respond::ok(ver),
        Err(e) => respond::err(e)
    }
}

pub async fn get_by_user_id(db: Data<Db>, user_id: Path<Uuid>) -> impl Responder {
    match VerificationRequest::get(&db.pool, user_id.into_inner()).await {
        Ok(ver) => respond::ok(ver),
        Err(e) => respond::err(e)
    }
}

pub async fn delete_by_user_id(db: Data<Db>, user_id: Path<Uuid>) -> impl Responder {
    match VerificationRequest::delete(&db.pool, user_id.into_inner()).await {
        Ok(ver) => respond::ok(ver),
        Err(e) => respond::err(e)
    }
}
