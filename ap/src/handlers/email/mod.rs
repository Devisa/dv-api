// use ap_com::proc::actions::email::notifier::{EmailRequest, Mailer};
use time::{Duration, OffsetDateTime};
use crate::{
    db::Db, auth::jwt, util::respond,
};
use actix_web::cookie::Cookie;
use actix_web::{
    HttpRequest, HttpResponse, Responder, get, post, web::{self, ServiceConfig}, cookie,
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg;
        /* .route("", web::post().to(send_message))
        .route("/verification/{user_id}", web::post().to(send_user_verification)); */
}

/* pub async fn send_message(db: web::Data<Db>, msg: web::Json<EmailRequest>) -> impl Responder {
    let msg= msg.into_inner();
    match Mailer::new(
        &msg.from.unwrap_or_default(), &msg.to.unwrap_or_default(),
        msg.reply_to, msg.subj, msg.body
    ).send() {
        Ok(_) => respond::ok(()),
        Err(e) => respond::err(e),
    }
}
pub async fn send_user_verification(db: web::Data<Db>, msg: web::Json<EmailRequest>) -> impl Responder {
    let msg= msg.into_inner();
    match Mailer::new(
        &msg.from.unwrap_or_default(), &msg.to.unwrap_or_default(),
        msg.reply_to, msg.subj, msg.body
    ).send() {
        Ok(_) => respond::ok(()),
        Err(e) => respond::err(e),
    }
} */
