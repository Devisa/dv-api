use crate::{db::Db, util::respond};
use uuid::Uuid;
use api_common::models::{Model, group::{Group, GroupUser}};
use sqlx::{prelude::*, postgres::Postgres};
use actix_web::{
    http::StatusCode,
    delete, get, post, web::{Json, Path, Data, HttpRequest,  ServiceConfig, self}, Responder
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_group))
        )
        .service(web::scope("/{group_id}").configure(individual_group_ops));
}

pub fn individual_group_ops(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_by_id))
            .route(web::post().to(update_by_id))
            .route(web::delete().to(delete_by_id))
        )
        .service(web::scope("/user")
            .service(web::resource("")
                .route(web::get().to(get_group_users))
                .route(web::post().to(add_group_user))
            )
            .service(web::scope("/{user_id}")
                .route("", web::get().to(get_group_user_links))
                .route("", web::post().to(add_group_user_link))
                .route("", web::delete().to(delete_group_user_link))
                .route("/add", web::post().to(add_group_member))
            )
        )
        .route("/name", web::put().to(update_name))
        .route("/description", web::put().to(update_description))
        ;
}

pub async fn new_group(db: Data<Db>, group: Json<Group>) -> impl Responder {
    match group.into_inner().insert(&db.pool).await {
        Ok(group) => respond::ok(group),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Group::get_all(&db.pool).await {
        Ok(groups) => respond::ok(groups),
        Err(e) => respond::err(e),
    }
}

pub async fn get_by_id(db: Data<Db>, id: Path<Uuid>) -> impl Responder {
    match Group::get(&db.pool, id.into_inner()).await {
        Ok(Some(group)) => respond::found(group),
        Ok(None) => respond::not_found("No group with id "),
        Err(e) => respond::err(e),
    }
}
pub async fn delete_by_id(db: Data<Db>, id: Path<Uuid>) -> impl Responder {
    match Group::delete(&db.pool, id.into_inner()).await {
        Ok(Some(group)) => respond::found(group),
        Ok(None) => respond::not_found("No group with id"),
        Err(e) => respond::err(e),
    }
}
pub async fn update_name(db: Data<Db>, id: Path<Uuid>, name: web::Query<String>) -> impl Responder {
    match Group::update_name(&db.pool, id.into_inner(), name.into_inner()).await {
        Ok(Some(group)) => respond::found(group),
        Ok(None) => respond::not_found("No group with id"),
        Err(e) => respond::err(e),
    }
}
pub async fn update_description(db: Data<Db>, id: Path<Uuid>, description: web::Query<String>) -> impl Responder {
    match Group::update_description(&db.pool, id.into_inner(), description.into_inner()).await {
        Ok(Some(group)) => respond::found(group),
        Ok(None) => respond::not_found("No group with id"),
        Err(e) => respond::err(e),
    }
}

// #[get("/{group_id}")]
pub async fn update_by_id() -> impl Responder {
    "Group ID".to_string()
}

pub async fn get_group_users(db: Data<Db>, id: Path<Uuid>) -> impl Responder {
    match Group::get_all_users(&db.pool, id.into_inner()).await {
        Ok(gu) => respond::ok(gu),
        Err(e) => respond::err(e),
    }
}
pub async fn add_group_user(db: Data<Db>, gu: Json<GroupUser>) -> impl Responder {
    match gu.into_inner().insert(&db.pool).await {
        Ok(gu) => respond::ok(gu),
        Err(e) => respond::err(e),
    }
}
pub async fn add_group_member(db: Data<Db>, path: Path<(Uuid, Uuid)>) -> impl Responder {
    let (group_id, user_id) = path.into_inner();
    match Group::add_member(&db.pool, group_id, user_id).await {
        Ok(gu) => respond::ok(gu),
        Err(e) => respond::err(e),
    }
}
pub async fn add_group_user_link() -> impl Responder {
    "Group ID".to_string()
}
pub async fn get_group_user_links() -> impl Responder {
    "Group ID".to_string()
}
pub async fn delete_group_user_link() -> impl Responder {
    "Group ID".to_string()
}
