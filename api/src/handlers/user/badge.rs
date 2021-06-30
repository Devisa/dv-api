use api_db::db::Db;
use crate::{util::respond, auth::jwt};
use api_common::models::user::{
    session::Session,
    credentials::Credentials,
    verification::VerificationRequest
};
use sqlx::{prelude::*, postgres::Postgres};
use actix_web::{
    Responder, HttpRequest, HttpResponse, HttpResponseBuilder, get,
    http::StatusCode, post,
    web::{self, ServiceConfig}
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
        );
}

