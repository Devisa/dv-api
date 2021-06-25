use api_common::models::Model;
use uuid::Uuid;
use crate::{
    db::Db,
    util::respond,
};
use api_common::models::post::Post;
use actix_web::{Responder, web::{Json, Data, Path, self, ServiceConfig}};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(new_post))
        )
        .service(web::scope("/group")
            .service(web::scope("/{group_id}")
                .route("", web::get().to(get_all_posts_in_group))
                .route("", web::post().to(new_post_in_group))
            )
        )
        .service(web::scope("/record")
            .service(web::scope("/{record_id}")
                .route("", web::get().to(get_all_posts_in_record))
                .route("", web::post().to(new_post_in_record))
            )
        )
        .service(web::scope("/topic")
            .service(web::scope("/{topic_id}")
                .route("", web::get().to(get_all_posts_in_topic))
                .route("", web::post().to(new_post_in_topic))
            )
        )
        .service(web::scope("/{post_id}")
            .service(web::resource("")
                .route(web::get().to(get_by_id))
                .route(web::post().to(update_by_id))
                .route(web::delete().to(delete_by_id))
            )
        )
        .service(web::scope("/user")
            .service(web::scope("/{user_id}")
                .route("", web::get().to(get_all_posts_by_user))
                .route("", web::post().to(new_post_in_user))
            )
        );
}

pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Post::get_all(&db.pool).await {
        Ok(p) => respond::ok(p),
        Err(e) => respond::err(e)
    }
}
pub async fn new_post(db: Data<Db>, post: Json<Post>) -> impl Responder {
    match post.into_inner().insert(&db.pool).await {
        Ok(p) => respond::ok(p),
        Err(e) => respond::err(e)
    }
}
pub async fn get_all_posts_by_user(db: Data<Db>, user_id: Path<Uuid>) -> impl Responder {
    match Post::get_all_by_user(&db.pool, user_id.into_inner()).await {
        Ok(p) => respond::ok(p),
        Err(e) => respond::err(e)
    }
}
pub async fn new_post_in_user(user_id: Path<Uuid>) -> impl Responder {
    format!("POST /post/user/{}", user_id.into_inner())
}
pub async fn get_all_posts_in_topic(db: Data<Db>, topic_id: Path<Uuid>) -> impl Responder {
    match Post::get_in_topic(&db.pool, topic_id.into_inner()).await {
        Ok(p) => respond::ok(p),
        Err(e) => respond::err(e)
    }
}
pub async fn new_post_in_topic(db: Data<Db>, topic_id: Path<Uuid>, post: Json<Post>) -> impl Responder {
    match post.into_inner().insert(&db.pool).await {
        Ok(p) => {
            match p.add_topic(&db.pool, topic_id.into_inner(), None).await {
                Ok(post_topic) => respond::ok(post_topic),
                Err(e) => respond::err(e),
            }
        },
        Err(e) => respond::err(e),
    }
}
pub async fn new_post_in_group(db: Data<Db>, group_id: Path<Uuid>, post: Json<Post>) -> impl Responder {
    match post.into_inner().insert(&db.pool).await {
        Ok(p) => {
            match p.add_group(&db.pool, group_id.into_inner()).await {
                Ok(post_topic) => respond::ok(post_topic),
                Err(e) => respond::err(e),
            }
        },
        Err(e) => respond::err(e),
    }
}

pub async fn get_all_posts_in_record(db: Data<Db>, topic_id: Path<Uuid>, post: Json<Post>) -> impl Responder {
    "".to_string()
}
pub async fn get_all_posts_in_group(db: Data<Db>, group_id: Path<Uuid>) -> impl Responder {
    match Post::get_in_group(&db.pool, group_id.into_inner()).await {
        Ok(p) => respond::ok(p),
        Err(e) => respond::err(e)
    }
}
pub async fn new_post_in_record() -> impl Responder {
    "".to_string()
}
pub async fn get_by_id(db: Data<Db>, post_id: Path<Uuid>) -> impl Responder {
    match Post::get(&db.pool, post_id.into_inner()).await {
        Ok(p) => respond::ok(p),
        Err(e) => respond::err(e)
    }
}
pub async fn update_by_id() -> impl Responder {
    "".to_string()
}
pub async fn delete_by_id(db: Data<Db>, post_id: Path<Uuid>) -> impl Responder {
    match Post::delete(&db.pool, post_id.into_inner()).await {
        Ok(p) => respond::ok(p),
        Err(e) => respond::err(e)
    }
}
