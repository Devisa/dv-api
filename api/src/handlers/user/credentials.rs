use uuid::Uuid;
use api_common::models::Model;
use crate::{db::Db, util::respond, auth::jwt};
use api_common::models::{Session, credentials::Credentials, verification::VerificationRequest};
use actix_web::{Responder, HttpRequest, HttpResponse, HttpResponseBuilder, get, http::StatusCode, post, web::{self, ServiceConfig, Path, Json, Data}};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_credentials))
        )
        .service(web::resource("/{credentials_id}")
            .route(web::delete().to(delete_by_id))
            .route(web::get().to(get_by_id))
        )
        .service(web::resource("/user/{user_id}")
            .route(web::get().to(get_by_user_id))
        )
        .service(web::scope("/user")
        );
}

pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Credentials::get_all(&db.pool).await {
        Ok(c) => respond::ok(c),
        Err(e) => respond::err(e),
    }
}
pub async fn new_credentials(db: Data<Db>, creds: Json<Credentials>) -> impl Responder {
    let c = creds.into_inner();
    match Credentials::create(c.user_id, c.username, c.password).insert(&db.pool).await {
        Ok(c) => respond::ok(c),
        Err(e) => respond::err(e),
    }
}
pub async fn get_by_id(db: Data<Db>, id: Path<Uuid>) -> impl Responder {
    match Credentials::get(&db.pool, id.into_inner()).await {
        Ok(c) => respond::ok(c),
        Err(e) => respond::err(e),
    }
}
pub async fn get_by_user_id(db: Data<Db>, user_id: Path<Uuid>) -> impl Responder {
    match Credentials::get(&db.pool, user_id.into_inner()).await {
        Ok(c) => respond::ok(c),
        Err(e) => respond::err(e),
    }
}
pub async fn delete_by_id(db: Data<Db>, id: Path<Uuid>) -> impl Responder {
    match Credentials::delete(&db.pool, id.into_inner()).await {
        Ok(c) => respond::ok(c),
        Err(e) => respond::err(e),
    }
}
pub async fn new_post_by_user() -> impl Responder {
    "".to_string()
}
pub async fn get_all_posts_in_topic() -> impl Responder {
    "".to_string()
}
pub async fn get_all_posts_in_record() -> impl Responder {
    "".to_string()
}
pub async fn get_all_posts_in_group() -> impl Responder {
    "".to_string()
}
pub async fn new_post_in_topic_inbox() -> impl Responder {
    "".to_string()
}
pub async fn new_post_in_record_inbox() -> impl Responder {
    "".to_string()
}
pub async fn new_post_in_group_inbox() -> impl Responder {
    "".to_string()
}
