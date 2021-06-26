pub mod redis;
use api_db::{Db, Model, Id};
use crate::util::respond;
use api_common::models::Session;
use actix_web::{
    Responder, HttpRequest, HttpResponse,
    web::{self, Json, Data, Path, ServiceConfig}
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("", web::to(root_handler))
        .route("", web::get().to(get_all))
        .service(web::scope("/{id}").configure(routes_id))
        .service(web::scope("/user").configure(routes_user))
        .service(web::scope("/redis").configure(redis::routes));
}

pub async fn root_handler(r: HttpRequest) -> impl Responder {
    use actix_web::http::Method;
    match r.method() {
        &Method::GET => {
            HttpResponse::Ok().body("GET /profile")
        },
        &Method::POST => {
            HttpResponse::Ok().body("POST /profile")
        },
        _ => HttpResponse::BadGateway().finish(),
    }
}

pub fn routes_id(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("/{id}")
            .service(web::resource("")
                .route(web::get().to(get_by_id))
                .route(web::post().to(add_by_id))
                .route(web::put().to(update_by_id))
                .route(web::delete().to(delete_by_id))
            )
            .service(web::resource("/status")
                .route(web::get().to(get_status_by_id))
                .route(web::post().to(update_status_by_id))
            )
        );
}
pub fn routes_user(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("/{user_id}")
            .service(web::resource("")
            .route(web::get().to(get_by_user_id))
            .route(web::post().to(add_by_user_id))
            .route(web::put().to(update_by_user_id))
            .route(web::delete().to(delete_by_user_id))

            )
            .service(web::resource("/status")
                .route(web::get().to(get_status_by_user_id))
                .route(web::post().to(update_status_by_user_id))
            )
        );
}
pub async fn get_all(req: HttpRequest, db: Data<Db>) -> impl Responder {
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
pub async fn add_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
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
pub async fn get_status_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    format!("GET /user/session/{}/status", &user_id)
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
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_get_all_users() {
        let app = App::new().route("/user/session", web::get().to(get_all));
        let mut serv = test::init_service(app).await;
        // let req = test::TestRequest::web::get().uri("/user/session/").to_request();
        // assert!(resp.status().is_ok())
    }
}
