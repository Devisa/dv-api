use crate::{ApiError, ApiResult};
use api_common::types::Expiration;
use api_common::models::{
    User
};
use api_db::{Model, Db, Id};
use actix_web::{
    App, dev,
    test::{TestRequest, self},
    dev::Server,
};

pub async fn db() -> sqlx::Result<Db> {
    let db_url = dotenv::var("DATABASE_URL")
        .unwrap();
    Ok(Db::new(&db_url).await.unwrap())
}

pub fn new_user(name: &str, email: &str) -> User {
    User::new(Some(name.to_string()), Some(email.to_string()), None)

}
pub async fn del_user(db: &Db, id: Id) -> anyhow::Result<Option<User>> {
    User::delete(&db.pool, id).await
}
pub async fn clear_users(db: &Db) -> anyhow::Result<()> {
    User::delete_all(&db.pool).await?;
    Ok(())
}

pub async fn add_user(db: &Db, name: &str, email: &str) -> sqlx::Result<User> {
    User::new(Some(name.to_string()), Some(email.to_string()), None)
        .insert(&db.pool)
        .await
}

pub async fn service(route_str: &str, route: actix_web::Route) -> impl actix_web::dev::Service<actix_http::Request> {
    test::init_service(App::new()
        .data(db())
        .route(route_str, route))
        .await
}
