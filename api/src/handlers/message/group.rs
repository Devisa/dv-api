use crate::util::respond;
use api_common::models::{
    group::Group,
    messages::DirectGroupMessage,
};
use api_db::{Model, Id, Db};
use actix_web::web::{Json, Data, Path, HttpRequest,  ServiceConfig, self};
use actix_web::Responder;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_group_dm))
        )
        .service(web::scope("/reply")
            .route("/{message_id}", web::post().to(reply_to_group_dm))
        )
        .service(web::scope("/sent/{sender_id}")
            .service(web::resource("")
                .route(web::get().to(get_all_from_sender))
                .route(web::post().to(new_group_dm_from_sender))
            )
        )
        .service(web::scope("/recv/{group_id}")
            .service(web::resource("")
                .route(web::get().to(get_all_to_group))
                .route(web::post().to(new_single_group_dm))
            )
            .service(web::resource("/{sender_id}")
                .route(web::get().to(all_from_sender_to_group))
                .route(web::post().to(new_single_group_dm_from_sender))
            )
        );
}
pub async fn new_group_dm(
    db: Data<Db>,
    group_dm: Json<DirectGroupMessage>
) -> impl Responder
{
    match group_dm.send(&db.pool).await {
        Ok(msg) => respond::ok(msg),
        Err(e) => respond::err(e),
    }
}
pub async fn reply_to_group_dm(
    db: Data<Db>,
    group_dm: Json<DirectGroupMessage>,
    replies_to_id: Path<Id>,
) -> impl Responder
{
    match DirectGroupMessage::reply_to(&db.pool,
        replies_to_id.into_inner(),
        group_dm.clone().sender_id,
        group_dm.clone().group_id,
        group_dm.content.clone(),
    ).await {
        Ok(msg) => respond::ok(msg),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all(db: Data<Db>) -> impl Responder {
    match DirectGroupMessage::get_all(&db.pool).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_to_group(db: Data<Db>, group_id: Path<Id>) -> impl Responder {
    match DirectGroupMessage::sent_to_group_id(&db.pool, group_id.into_inner()).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_from_sender(db: Data<Db>, sender_id: Path<Id>) -> impl Responder {
    match DirectGroupMessage::sent_by_sender_id(&db.pool, sender_id.into_inner()).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_thread_starters(db: Data<Db>) -> impl Responder {
    match DirectGroupMessage::get_all_thread_starters(&db.pool).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_replies(db: Data<Db>) -> impl Responder {
    match DirectGroupMessage::get_all_non_thread_starters(&db.pool).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn all_from_sender_to_group(db: Data<Db>, path: Path<(Id, Id)>) -> impl Responder {
    let ( group_id, sender_id ) = path.into_inner();
    match DirectGroupMessage::all_from_sender_to_group(&db.pool, sender_id, group_id).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn new_single_group_dm(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn new_group_dm_from_sender(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn get_all_group_dms_from_sender(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn new_single_group_dm_from_sender(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn get_single_group_dms_from_sender(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
