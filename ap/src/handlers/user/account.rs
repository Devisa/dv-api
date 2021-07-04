use crate::util::respond;
use ap_com::{Db, Id, Model};
use ap_com::{
    models::user::account::{Account, AccountQuery},
    types::auth::{Provider, ProviderType},
};
use actix_web::{
    HttpRequest, HttpResponse, Responder,
    web::{self, Path, Data, Json, ServiceConfig}
};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg
        .service(web::scope("").configure(routes_crud))
        .service(web::scope("/provider").configure(routes_provider))
        .service(web::scope("/id/{account_id}").configure(routes_id))
        .service(web::scope("/user_id/{user_id}").configure(routes_user_id));
}
// NOTE: Add route to create by provider? Probably useless
fn routes_provider(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("/{provider_id}")
            .route(web::get().to(get_all_by_provider))
            .route(web::delete().to(delete_all_by_provider))
        )
        .service(web::resource("/type/{provider_type}")
            .route(web::get().to(get_all_by_provider_type))
            .route(web::delete().to(delete_all_by_provider_type))
        );
}

fn routes_crud(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all))
            .route(web::post().to(add_new))
            .route(web::put().to(update))
            .route(web::delete().to(delete_all))
        );
}

pub fn routes_id(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_by_id))
            .route(web::post().to(add_by_id))
            .route(web::put().to(update_by_id))
            .route(web::delete().to(delete_by_id))
        );
}

pub fn routes_user_id(cfg: &mut ServiceConfig) {
    cfg
        .service(web::resource("")
            .route(web::get().to(get_all_by_user_id))
            .route(web::post().to(add_by_user_id))
            .route(web::put().to(update_by_user_id))
            .route(web::delete().to(delete_all_by_user_id))
        );
}

pub async fn get_all(db: Data<Db>) -> impl Responder {
    match Account::get_all(&db.pool).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}

pub async fn update(db: Data<Db>, account: Json<Account>) -> impl Responder {
    "PUT /user/account".to_string()
}

pub async fn delete_all(db: Data<Db>) -> impl Responder {
    match Account::delete_all(&db.pool).await {
        Ok(accts) => respond::ok(accts),
        Err(e) => respond::err(e)
    }
}
pub async fn get_by_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Account::get_all_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}
pub async fn add_by_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Account::delete_all_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}

pub async fn delete_by_id(db: Data<Db>, id: Path<Id>) -> impl Responder {
    match Account::delete(&db.pool, id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}

pub async fn add_new(db: Data<Db>, account: Json<Account>) -> impl Responder {
    match account.into_inner().insert(&db.pool).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e)
    }
}

pub async fn update_by_id(db: Data<Db>, account: Json<Account>, id: Path<Id>) -> impl Responder {
    "PUT /user/account/id/{}".to_string()
}

pub async fn get_all_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Account::get_all_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}
// TODO unimplemented
pub async fn update_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Account::delete_all_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}
// TODO unimplemented
pub async fn add_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Account::delete_all_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}

pub async fn delete_all_by_user_id(db: Data<Db>, user_id: Path<Id>) -> impl Responder {
    match Account::delete_all_by_user_id(&db.pool, user_id.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_by_provider(db: Data<Db>, prov: Path<Provider>) -> impl Responder {
    match Account::get_all_by_provider(&db.pool, prov.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}
pub async fn delete_all_by_provider(db: Data<Db>, prov: Path<Provider>) -> impl Responder {
    match Account::delete_all_by_provider(&db.pool, prov.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}
pub async fn get_all_by_provider_type(db: Data<Db>, ptype: Path<ProviderType>) -> impl Responder {
    match Account::get_all_by_provider_type(&db.pool, ptype.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}
pub async fn delete_all_by_provider_type(db: Data<Db>, ptype: Path<ProviderType>) -> impl Responder {
    match Account::delete_all_by_provider_type(&db.pool, ptype.into_inner()).await {
        Ok(acct) => respond::ok(acct),
        Err(e) => respond::err(e),
    }
}

// impl Responder for Account {
//     fn respond_to(self, req: &HttpRequest) -> HttpResponse {
//         respond::ok(serde_json::to_value(self))

//     }
// }
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::prelude::*;
    use crate::test::*;
    use ap_com::{types::*, models::{Profile, Credentials, User}};

    #[actix::test]
    async fn insert_account_ok() -> sqlx::Result<()> {
        let db = db().await.unwrap();
        let cid = uuid::Uuid::new_v4();
        let user = User::gen().insert(&db.pool).await.unwrap();
        let creds = Credentials {
            id: Id::new(cid),
            user_id: user.clone().id,
            username: user.clone().name.unwrap(),
            password: user.clone().name.unwrap(),
        }
            .insert(&db.pool)
            .await.unwrap();
        let acct = Account {
            id: Id::gen(),
            user_id: user.clone().id,
            provider_account_id: Id::new(cid),
            provider_id: Provider::Devisa,
            provider_type: ProviderType::Credentials,
            access_token: None,
            refresh_token: None,
            access_token_expires: None,
            created_at: now(),
            updated_at: now(),
        }
        .insert(&db.pool).await?;
        Ok(())
    }
}
