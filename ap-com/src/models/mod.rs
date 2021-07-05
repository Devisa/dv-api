//! This module handles model routes andhandler functions
//!    and re-exports models within
pub mod category;
pub mod action;
pub mod automata;
pub mod ai;
pub mod messages;
pub mod condition;
pub mod channel;
pub mod topic;
pub mod book;
pub mod link;
pub mod user;
pub mod group;
pub mod post;
pub mod task;
pub mod item;
pub mod field;
pub mod record;

use chrono::NaiveDateTime;
pub use user::{User,
    session::Session,
    credentials::Credentials,
    verification::VerificationRequest,
    profile::Profile,
    account::Account
};
pub use record::{RecordRelation, Record};
pub use link::Link;
pub use item::{Item, ItemRelation};
pub use field::{FieldRelation, Field};
pub use post::Post;
pub use group::Group;
pub use topic::Topic;
pub use action::Action;
pub use category::Category;
pub use automata::Automata;
pub use messages::{
    DirectUserMessage,
    DirectGroupMessage,
    DirectTopicMessage,
    DirectGroupMessageReadReceipt
};
// pub use learn::LearningUnit;
// pub use book::{UserBook, RecordBook, GroupBook, TopicBook};
// pub use condition::Condition;

use crate::{Db, util::respond, Id};
use std::fmt::Debug;
use actix_web::{HttpResponse, web::{self, Path, Data, Json, ServiceConfig}, Resource, Scope};
use serde::{Serialize, Deserialize};
use sqlx::{
    prelude::*, FromRow, Postgres, PgPool,
    postgres::PgRow,
};

#[async_trait::async_trait]
pub trait Model
where
    for<'a> Self: 'static + Sized + FromRow<'a, PgRow> + Unpin + Send + Debug + PartialEq + Serialize + Deserialize<'a> {

    /// Return corresponding table string
    fn table() -> String;

    /// Returns the model's Id if it has one, otherwise generates new ID
    fn id(self) -> Id {
        Id::gen()
    }

    /// Returns what this field's id would be called as a foreign key on another table
    ///     e.g. user_id for users
    ///     Overwrite for tables where this doesn't apply -- ex. categories vs. category_id
    fn id_str() -> String {
        let mut out = Self::table();
        out.pop();
        out.push_str("_id");
        return out;
    }


    /// Insert the model into the database
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self>;

    /// The relative path at which the scope service is provided
    ///     Relative path is provided in the scope parameter.
    fn path() -> String {
        let mut path = String::new();
        path.push('/');
        path.push_str(Self::table().as_str());
        path.pop();
        path.to_string()
    }

    /// Encapsulated service for model's routes, served at Self::path()
    fn scope() -> actix_web::Scope {
        actix_web::web::scope(Self::path().as_str())
            .configure(Self::routes)
            .service(Self::crud_routes())
            .service(Self::by_id_routes())
    }

    fn by_id_routes() -> actix_web::Resource {
        web::resource("/id/{id}")
            .route(web::get().to(Self::service_get_by_id))
            .route(web::delete().to(Self::service_delete_by_id))
    }

    fn crud_routes() -> actix_web::Resource {
        web::resource("")
            .route(web::get().to(Self::service_get_all))
            .route(web::post().to(Self::service_add_new))
            .route(web::delete().to(Self::service_delete_all))
            .route(web::put().to(Self::service_update))
    }

    /// Meant to be implemented by user for non-linked based routes
    fn routes(cfg: &mut ServiceConfig) {
        cfg;
    }

    async fn service_get_all(db: Data<Db>) -> actix_web::Result<HttpResponse> {
        match Self::get_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_delete_all(db: Data<Db>) -> actix_web::Result<HttpResponse> {
        match Self::delete_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_delete_by_id(db: Data<Db>, id: Path<Id>) -> actix_web::Result<HttpResponse> {
        match Self::delete_by_id(&db.pool, id.into_inner()).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_add_new(db: Data<Db>, model: Json<Self>) -> actix_web::Result<HttpResponse> {
        match model.into_inner().insert(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    async fn service_get_by_id(db: Data<Db>, id: Path<Id>) -> actix_web::Result<HttpResponse> {
        match Self::get(&db.pool, id.into_inner()).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }
    //TODO implement
    async fn service_update(db: Data<Db>) -> actix_web::Result<HttpResponse> {
        match Self::get_all(&db.pool).await {
            Ok(model) => Ok(respond::ok(model)),
            Err(e) => Ok(respond::err(e)),
        }
    }

    async fn get(db: &PgPool, id: Id) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("SELECT * FROM {} WHERE id = $1", Self::table()))
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    async fn get_all(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("SELECT * FROM {}", Self::table()))
            .fetch_all(db).await?;
        Ok(res)
    }

    async fn delete(db: &PgPool, id: Id) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} WHERE id = $1 RETURNING *", Self::table()))
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    async fn delete_all(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} RETURNING *", Self::table()))
            .fetch_all(db).await?;
        Ok(res)
    }

    async fn get_by_id(self, db: &PgPool, kind: &str, id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("SELECT * FROM {} WHERE {}_id = $1", Self::table(), kind))
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    async fn delete_by_id(db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} WHERE id = $1", Self::table()))
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }
    async fn delete_by_id_kind<K: Model>(db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} WHERE {} = $1", Self::table(), K::table()))
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }
    async fn get_by_id_kind<K: Model>(db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("SELECT * FROM {} WHERE {} = $1", Self::table(), K::table()))
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }
    // TODO get from other table where Self::id_str() = ?

    async fn get_after(self, db: &PgPool, datetime: NaiveDateTime) -> sqlx::Result<Vec<Self>> {
        /* let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} WHERE {}_id = $1", Self::table(), kind))
            .bind(id)
            .fetch_all(db).await?; */
        Ok(Vec::new())
    }
}
#[async_trait::async_trait]
pub trait Relation<T>
where
    T: Model
{

}

#[async_trait::async_trait]
pub trait UserOneToOne: Model {

    async fn get_by_user_id(db: &PgPool, user_id: Id) -> sqlx::Result<Self>;
}

/// Trait for tables which map one to many users (i.e. group membership)
#[async_trait::async_trait]
pub trait UserOneToMany: Model {

    async fn get_by_user_id(db: &PgPool, user_id: Id) -> sqlx::Result<Vec<Self>>;

}

/// Trait for tables which map many to many users (i.e. group membership)
#[async_trait::async_trait]
pub trait UserManyToMany: Model {

    async fn get_by_user_id(db: &PgPool, user_id: Id) -> sqlx::Result<Vec<Self>>;

}

pub enum UserRelKind {
    OneUserToOne,
    OneUserToMany,
    ManyUsersToMany,
}

/*
impl<'r> sqlx::FromRow<'r, PgRow> for dyn Model {
} */

/* pub trait ManyToOne<T: Model>
where
    Self: Sized + for<'r> FromRow<'r, PgRow> + Unpin + Send ,
    T: Sized + for<'r> FromRow<'r, PgRow> + Unpin + Send
{

} */


pub async fn get<'r, F: 'r + FromRow<'r, PgRow>>() {

}

pub async fn get_all<'r, F: 'r + FromRow<'r, PgRow>>() {

}

pub async fn post<'r, F: 'r + FromRow<'r, PgRow>>() {

}
