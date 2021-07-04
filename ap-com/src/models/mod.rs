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
pub use automata::Automata;
pub use messages::{DirectUserMessage, DirectGroupMessage, DirectTopicMessage, DirectGroupMessageReadReceipt};
// pub use learn::LearningUnit;
// pub use book::{UserBook, RecordBook, GroupBook, TopicBook};
// pub use condition::Condition;

use crate::Id;
use sqlx::{
    prelude::*, FromRow, Postgres, PgPool,
    postgres::PgRow,
};

#[async_trait::async_trait]
pub trait Model
where
    Self: Sized + for<'r> FromRow<'r, PgRow> + Unpin + Send {

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

    async fn delete_by_id(self, db: &PgPool, kind: &str, id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} WHERE {}_id = $1", Self::table(), kind))
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

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
