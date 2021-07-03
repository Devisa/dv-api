// use api_common::prelude::User;
use api_db::{Db, Model, Id};
/* use api_common::types::{
    time::{ Expiration, ExpirationQuery },
    auth::{Provider, ProviderType,}
}; */
use actix_session::Session;
use crate::{ApiError, ApiResult};
use actix_web::{
    HttpRequest, HttpResponse, Responder, guard,
    web::{
        self, Data, Json, Path, Query, ServiceConfig
    }
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("/id/{id}")
        )
        .service(web::scope("/{uid}")
            .route("/{path}", web::post().to(new_session_on_path))
            .service(web::resource("")
                .route(web::get().to(get_by_uid))
                .route(web::post().to(add_by_uid))
                .route(web::delete().to(delete_by_uid))
            )
        )
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_session))
            .route(web::delete().to(clear_session))
        )
        .route("/check", web::to(check))
        .route("/new", web::to(new_session));
}

pub async fn get_all(uid: Path<Id>) -> ApiResult<impl Responder> {
    Ok(format!("PUT /user/session/{}", &uid))
}

pub async fn add_session(uid: Path<Id>) -> ApiResult<impl Responder> {
    Ok(format!("PUT /user/session/{}", &uid))
}
pub async fn clear_session(uid: Path<Id>) -> ApiResult<impl Responder> {
    Ok(format!("PUT /user/session/{}", &uid))
}

pub async fn get_by_uid(uid: Path<Id>) -> ApiResult<impl Responder> {
    Ok(format!("PUT /user/session/{}", &uid))
}
pub async fn add_by_uid(uid: Path<Id>) -> ApiResult<impl Responder> {
    Ok(format!("PUT /user/session/{}", &uid))
}
pub async fn delete_by_uid(uid: Path<Id>) -> ApiResult<impl Responder> {
    Ok(format!("PUT /user/session/{}", &uid))
}
pub async fn new_session(_session: Session, ) -> ApiResult<impl Responder> {
    Ok(format!(""))
}
pub async fn check(_session: Session, ) -> ApiResult<impl Responder> {
    Ok(format!(""))
}
pub async fn new_session_on_path(_session: Session, ) -> ApiResult<impl Responder> {
    Ok(format!(""))
}
