
use ap_com::models::Automata;
use time::{Duration, OffsetDateTime};
use crate::{
    db::Db, auth::jwt, util::respond,
};
use actix_web::cookie::Cookie;
use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post, web::{self, ServiceConfig}, cookie,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("", web::get().to(|| respond::ok_msg("GET /automata/")))
        .route("", web::post().to(|| respond::ok_msg("POST /automata")) )
        ;
}
