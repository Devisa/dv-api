use crate::util::respond;
use api_db::{Db, Id, Model};
use api_common::models::account::Account;
use actix_web::{
    HttpRequest, HttpResponse, Responder, get, http::StatusCode, post,
    web::{self, Path, Data, Json, ServiceConfig}
};

pub struct Accounts {
    pub id: Id,
}

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .route("", web::get().to(get_all))
        .service(web::resource("/{user_id}")
            .route(web::get().to(get_by_user_id))
            .route(web::delete().to(delete_by_user_id))
        );
}

pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Account::get_all(&db.pool).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}

pub async fn get_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Account::get_all_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}

pub async fn delete_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Account::delete_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}

// impl Responder for Account {
//     fn respond_to(self, req: &HttpRequest) -> HttpResponse {
//         respond::ok(serde_json::to_value(self))

//     }
// }
