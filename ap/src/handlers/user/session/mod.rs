// pub mod redis;
pub mod api;

use ap_com::{Db, Model, Id};
use ap_com::types::{
    time::{ Expiration, ExpirationQuery },
    auth::{Provider, ProviderType,}
};
use crate::util::respond;
use ap_com::models::Session;
use actix_web::{HttpRequest, HttpResponse, Responder, guard, web::{self, Data, Json, Path, Query, ServiceConfig}};


pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::put().to(update))
            .route(web::post().to(add_session))
            .route(web::delete().to(delete_all))
        )
        .service(web::scope("/api").configure(api::routes))
        .service(web::scope("/id/{id}").configure(routes_id))
        .service(web::scope("/userid/{user_id}").configure(routes_user_id));
        // .service(web::scope("/redis").configure(redis::routes));
}

pub fn routes_id(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_by_id))
            .route(web::post().to(add_by_id))
            .route(web::put().to(update_by_id))
            .route(web::delete().to(delete_by_id))
        )
        .service(web::resource("/status")
            .route(web::get().to(get_status_by_id))
            .route(web::post().to(update_status_by_id))
        );
}
pub fn routes_user_id(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_by_user_id))
            .route(web::post().to(add_by_user_id))
            .route(web::put().to(update_by_user_id))
            .route(web::delete().to(delete_by_user_id))
        )
        .service(web::resource("/status")
            .route(web::get().to(get_status_by_user_id))
            .route(web::post().to(update_status_by_user_id))
        );
}
pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Session::get_all(&db.pool).await {
        Ok(sess) => respond::ok(sess),
        Err(e) => respond::err(e)
    }
}
pub async fn delete_all(db: Data<Db>) -> impl Responder {
    match Session::get_all(&db.pool).await {
        Ok(sess) => respond::ok(sess),
        Err(e) => respond::err(e)
    }
}
pub async fn update(db: Data<Db>) -> impl Responder {
    match Session::get_all(&db.pool).await {
        Ok(sess) => respond::ok(sess),
        Err(e) => respond::err(e)
    }
}

pub async fn get_by_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Session::get(&db.pool, user_id.into_inner()).await {
        Ok(sess) => respond::ok(sess),
        Err(e) => respond::err(e)
    }
}
pub async fn get_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Session::get(&db.pool, user_id.into_inner()).await {
        Ok(sess) => respond::ok(sess),
        Err(e) => respond::err(e)
    }
}

pub async fn delete_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Session::delete(&db.pool, user_id.into_inner()).await {
        Ok(sess) => respond::ok(sess),
        Err(e) => respond::err(e)
    }
}

/// TODO fix this
pub async fn add_by_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    let id: Id = user_id.into_inner();
    match Session::create_two_day_session(&db.pool, id).await {
        Ok(sess) => respond::ok(sess),
        Err(e) => respond::err(e)
    }
}
pub async fn add_session(db: Data<Db>, sess: Json<Session>) -> impl Responder {
    match Session::create_two_day_session(&db.pool, sess.into_inner().user_id).await {
        Ok(sess) => respond::ok(sess),
        Err(err) => respond::err(err),
    }
}
pub async fn add_by_user_id(db: Data<Db>, user_id: Path<Id>, exp: Option<Query<ExpirationQuery>>) -> impl Responder {
    match Session::create_two_day_session(&db.pool, user_id.into_inner()).await {
        Ok(sess) => respond::ok(sess),
        Err(err) => respond::err(err),
    }
}
pub async fn update_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    format!("PUT /user/session/user/{}", &user_id)
}
pub async fn update_status_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    format!("PUT /user/session/user/status/{}", &user_id)
}
pub async fn update_status_by_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    format!("PUT /user/session/{}/status", &user_id)
}
pub async fn get_status_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Session::get(&db.pool, id.into_inner()).await {
        Ok(Some(sess)) => respond::ok(sess),
        Ok(None) => respond::not_found("None with that ID"),
        Err(e) => respond::err(e)
    }
}
pub async fn get_status_by_user_id(db: Data<Db>, user_id: Path<Id>) -> actix_web::Result<impl Responder> {
    Ok(format!("GET /user/session/{}/status", &user_id))
}
pub async fn delete_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Session::delete(&db.pool, id.into_inner()).await {
        Ok(sess) => respond::ok(sess),
        Err(e) => respond::err(e)
    }
}
pub async fn update_by_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    format!("PUT /user/session/{}", &user_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use actix_web::{test::{self, TestRequest}, web, };
    use ap_com::types::Expiration;

    async fn new(db: &Db, user_id: Id, exp: Option<Expiration>,) -> crate::ApiResult<Session> {
        let sess = Session::create(user_id, exp.unwrap_or_default()).unwrap()
            .insert(&db.pool)
            .await?;
        Ok(sess)
    }

    #[actix_rt::test]
    async fn get_all_sessions_ok() -> anyhow::Result<()> {
        let db = db().await?;
        let u1 = add_user(&db, "user1", "user1@email.com").await?;
        let u2 = add_user(&db, "user2", "user2@email.com").await?;
        let u3 = add_user(&db, "user3", "user3@email.com").await?;
        let v = vec![u1, u2, u3];
        let req = TestRequest::get();
        /* let resp = get_all(Data::new(db)).await
            .respond_to(&req);
        assert!(resp.status().is_ok()); */
        clear_users(&db).await?;
        Ok(())
    }

    #[actix_rt::test]
    async fn get_session_by_id_ok() -> crate::ApiResult<()> {
        let db = db().await?;
        let srv = service("/user/{id}", web::get().to(get_all)).await;
        let user1 = add_user(&db, "user1", "user1@email.com").await?;
        // let sess1 = new(&db, user1.id, Expiration::two_days()).await?;
        let req = TestRequest::get().uri("/user/1")
            .to_http_request();
        /* let resp = get_by_id(Data::new(db), Path::new(1)).await;
        assert!(resp.status().is_ok());
        clear_users(&db).await?; */
        Ok(())
    }

    #[actix_rt::test]
    async fn create_session_ok() -> anyhow::Result<()> {
        let db = db().await?;
        /* let user = add_user("user1", "user1@email.com").await;
        let session = Session::create(user.id, Expiration::two_days()).await?; */
        /* let srv = service("/user/session", web::post().to(add_session)).await;
        let req = TestRequest::post().uri("/user/session"); */
        Ok(())
    }
}

