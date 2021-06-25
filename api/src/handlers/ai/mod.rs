
use time::{Duration, OffsetDateTime};
use crate::{
    db::Db, auth::jwt, util::respond,
};
use actix_web::cookie::Cookie;
use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post, web::{self, ServiceConfig}, cookie,
};
use api_common::models::{Profile, account::AccountProvider, auth::CredentialsSignupIn, credentials::{CredentialsIn, Credentials}, user::{User, UserIn}};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("", web::get().to(|| HttpResponse::Ok().body("Hello from GET /ai")))
        .route("", web::post().to(|| HttpResponse::Ok().body("Hello from POST /ai")))
        .route("", web::delete().to(|| HttpResponse::Ok().body("Hello from DELET /ai")))
        ;
}
