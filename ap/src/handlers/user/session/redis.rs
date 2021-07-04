use crate::util::respond;
use redis::AsyncCommands;
use actix_web::{HttpRequest, HttpResponse, Responder, get, http::StatusCode, post, web::{self, ServiceConfig}};
// use crate::actors::redis::RedisActor;
use serde::{Serialize, Deserialize};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all_redis_sessions))
        )
        .service(web::resource("/{user_id}")
            .route(web::get().to(get_redis_session))
            .route(web::post().to(create_new_redis_session))
            .route(web::put().to(update_current_redis_session))
            .route(web::delete().to(terminate_redis_session))
        )
        ;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionQuery {
    pub user_id: i32
}

async fn get_all_redis_sessions(rd: web::Data<redis::Client>) -> impl Responder {
    let user_id: i32 = 2;
    let id_key = format!("id{}", &user_id.to_string());
    match rd.get_async_connection().await {
        Ok(mut conn) => match conn.get::<String, String>(id_key).await {
            Ok(sess) => respond::ok(sess),
            Err(e) => respond::err(e),
        },
        Err(e) => respond::err(e),
    }
}
async fn get_redis_session(rd: web::Data<redis::Client>, user_id: web::Path<i32>) -> impl Responder {
    let id_key = format!("id{}", &user_id.to_string());
    match rd.get_async_connection().await {
        Ok(mut conn) => match conn.get::<String, String>(id_key).await {
            Ok(sess) => respond::ok(sess),
            Err(e) => respond::err(e),
        },
        Err(e) => respond::err(e),
    }
}
async fn create_new_redis_session(rd: web::Data<redis::Client>, user_id: web::Path<i32>) -> impl Responder {
    let id_key = format!("id{}", &user_id.to_string());
    match rd.get_async_connection().await {
        Ok(mut conn) => match conn.set::<String, String, String>(id_key, user_id.to_string()).await {
            Ok(sess) => respond::ok(sess),
            Err(e) => respond::err(e),
        },
        Err(e) => respond::err(e),
    }
}
async fn update_current_redis_session(rd: web::Data<redis::Client>, user_id: web::Path<i32>) -> impl Responder {
    let id_key = format!("id{}", &user_id.to_string());
    match rd.get_async_connection().await {
        Ok(mut conn) => match conn.set::<String, String, String>(id_key, user_id.to_string()).await {
            Ok(sess) => respond::ok(sess),
            Err(e) => respond::err(e),
        },
        Err(e) => respond::err(e),
    }
}
async fn terminate_redis_session(
    rd: web::Data<redis::Client>,
    user_id: web::Path<i32>,
    req: HttpRequest,
    ) -> impl Responder {
    let id_key = format!("id{}", &user_id.to_string());
    match rd.get_async_connection().await {
        Ok(mut conn) => match conn.set::<String, String, String>(id_key, user_id.to_string()).await {
            Ok(sess) => respond::ok(sess),
            Err(e) => respond::err(e),
        },
        Err(e) => respond::err(e),
    }
}
