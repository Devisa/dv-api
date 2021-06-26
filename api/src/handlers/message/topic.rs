use api_db::{Db, Model, Id};
use crate::util::respond;
use api_common::models::{
    topic::Topic,
    messages::DirectTopicMessage,
};
use actix_web::{
    web::{Json, Path, Data, HttpRequest,  ServiceConfig, self}, Responder
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_topic_dm))
        )
        .service(web::scope("/{topic_id}")
            .route("/thread", web::get().to(get_all_topic_thread_starters))
            .route("/reply", web::get().to(get_all_topic_replies))
            .service(web::resource("")
                .route(web::get().to(get_topic_dm_by_id))
            )
        )
        .route("/thread", web::get().to(get_all_thread_starters))
        .route("/reply", web::get().to(get_all_replies))
        .route("/reply/{message_id}", web::post().to(reply_to_topic_dm))
        .service(web::scope("/sent/{sender_id}")
            .service(web::resource("")
                .route(web::get().to(get_all_topic_dms_from_sender))
                .route(web::get().to(new_topic_dm_from_sender))
            )
        )
        .service(web::scope("/recv/{topic_id}")
            .service(web::resource("")
                .route(web::get().to(get_single_topic_dms))
                .route(web::post().to(new_single_topic_dm))
            )
            .service(web::resource("/{sender_id}")
                .route(web::get().to(get_single_topic_dms_from_sender))
                .route(web::post().to(new_single_topic_dm_from_sender))
            )
        );
}
pub async fn new_topic_dm(
    db: Data<Db>,
    topic_dm: Json<DirectTopicMessage>
) -> impl Responder
{
    match DirectTopicMessage::new_thread(&db.pool,
        topic_dm.clone().sender_id,
        topic_dm.clone().topic_id,
        topic_dm.content.clone(),
    ).await {
        Ok(msg) => respond::ok(msg),
        Err(e) => respond::err(e),
    }
}
pub async fn reply_to_topic_dm(
    db: Data<Db>,
    topic_dm: Json<DirectTopicMessage>,
    replies_to_id: Path<Id>,
) -> impl Responder
{
    match DirectTopicMessage::reply_to(&db.pool,
        replies_to_id.into_inner(),
        topic_dm.clone().sender_id,
        topic_dm.clone().topic_id,
        topic_dm.content.clone(),
    ).await {
        Ok(msg) => respond::ok(msg),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all(db: Data<Db>) -> impl Responder {
    match DirectTopicMessage::get_all(&db.pool).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_thread_starters(db: Data<Db>) -> impl Responder {
    match DirectTopicMessage::get_all_thread_starters(&db.pool).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_replies(db: Data<Db>) -> impl Responder {
    match DirectTopicMessage::get_all_replies(&db.pool).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_topic_thread_starters(db: Data<Db>, topic_id: Path<Id>) -> impl Responder {
    match DirectTopicMessage::get_all_topic_thread_starters(&db.pool, topic_id.into_inner()).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_topic_replies(db: Data<Db>, topic_id: Path<Id>) -> impl Responder {
    match DirectTopicMessage::get_all_topic_replies(&db.pool, topic_id.into_inner()).await {
        Ok(messages) => respond::ok(messages),
        Err(e) => respond::err(e),
    }
}
pub async fn get_single_topic_dms(db: Data<Db>, topic: Json<Topic>) -> impl Responder {
    "".to_string()
}
pub async fn new_single_topic_dm(db: Data<Db>, topic: Json<Topic>) -> impl Responder {
    "".to_string()
}
pub async fn new_topic_dm_from_sender(db: Data<Db>, topic: Json<Topic>) -> impl Responder {
    "".to_string()
}
pub async fn get_all_topic_dms_from_sender(db: Data<Db>, topic: Json<Topic>) -> impl Responder {
    "".to_string()
}
pub async fn get_single_topic_dms_from_sender(db: Data<Db>, topic: Json<Topic>) -> impl Responder {
    "".to_string()
}
pub async fn new_single_topic_dm_from_sender(db: Data<Db>, topic: Json<Topic>) -> impl Responder {
    "".to_string()
}
pub async fn get_topic_dm_by_id(db: Data<Db>, topic: Json<Topic>) -> impl Responder {
    "".to_string()
}

