use ap_com::{Db, Model, Id};
use crate::util::respond;
use ap_com::models::{DirectUserMessage, group::Group};
use actix_web::{
    web::{Json, Data, Path, HttpRequest,  ServiceConfig, self}, Responder
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all_user_dms))
            .route(web::post().to(new_user_dm))
        )
        .service(web::scope("/reply")
            .route("/{message_id}", web::post().to(reply_to_user_dm))
        )
        .service(web::scope("/sent/{sender_id}")
            .service(web::resource("")
                .route(web::get().to(get_all_user_dms_from_sender))
                .route(web::get().to(new_user_dm_from_sender))
            )
        )
        .service(web::scope("/recv/{user_id}")
            .service(web::resource("")
                .route(web::get().to(get_single_user_dms))
                .route(web::post().to(new_single_user_dm))
            )
            .service(web::resource("/{sender_id}")
                .route(web::get().to(get_dms_between_users))
                .route(web::get().to(new_dm_between_users))
            )
        );
}
pub async fn get_all_user_dms(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn new_user_dm(
    db: Data<Db>,
    user_dm: Json<DirectUserMessage>
) -> impl Responder
{
    match DirectUserMessage::new(
        user_dm.clone().sender_id,
        user_dm.clone().recipient_id,
        user_dm.clone().content.clone(),
    ).send(&db.pool).await {
        Ok(msg) => respond::ok(msg),
        Err(e) => respond::err(e),
    }
}
pub async fn reply_to_user_dm(
    db: Data<Db>,
    user_dm: Json<DirectUserMessage>,
    replies_to_id: Path<Id>,
) -> impl Responder
{
    match DirectUserMessage::reply_to(&db.pool,
        replies_to_id.into_inner(),
        user_dm.clone().sender_id,
        user_dm.clone().recipient_id,
        user_dm.content.clone(),
    ).await {
        Ok(msg) => respond::ok(msg),
        Err(e) => respond::err(e),
    }
}
pub async fn get_single_user_dms(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn new_single_user_dm(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn new_user_dm_from_sender(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn get_all_user_dms_from_sender(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn get_dms_between_users(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
pub async fn new_dm_between_users(db: Data<Db>, group: Json<Group>) -> impl Responder {
    "".to_string()
}
