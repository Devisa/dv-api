//! The category model
//!
use actix::prelude::*;
use actix_web::web::{self, get, ServiceConfig, Path};
use crate::{
    util::respond,
    models::topic::{Topic, TopicCategory}, Id, Db, Model,
    rel::{LinkedTo, Linked},
};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::{PgPool, FromRow, Postgres, types::Json};
use uuid::Uuid;
use crate::{now, private, Status};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Category {
    #[serde(default = "Id::gen")]
    pub id: Id,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Category {

    pub fn new(name: String, description: Option<String>) -> Self {
        let cat = Self {
            name, id: Id::gen(), description,
            created_at: now(), updated_at: now()
        };
        cat
    }


    pub async fn get_all(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("SELECT * FROM categories")
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn get_by_id(db: &PgPool, id: Id) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("SELECT * FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }
    pub async fn delete_by_id(db: &PgPool, id: Id) -> sqlx::Result<Option<Id>> {
        let res = sqlx::query_scalar("DELETE FROM categories WHERE id = $1 ")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }
    pub async fn linked_to_topic(db: &PgPool, topic_id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM categories
            INNER JOIN topic_categories
            ON categories.id = topic_categories.category_id
            WHERE topic_categories.topic_id = $1
        ")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    pub async fn get_posts(db: &PgPool, topic_id: Id) -> sqlx::Result<Vec<Topic>>
    {
        let res = sqlx::query_as::<Postgres, Topic>("
            SELECT * FROM topics
            INNER JOIN topic_posts ON posts.id = topic_posts.post_id
            INNER JOIN topics on topics.id = topic_posts.topic_id
            WHERE topics.id = $1
        ")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl Model for Category {
    #[inline]
    fn path() -> String { String::from("/category") }

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /category/hi".to_string())))
            .service(<Category as LinkedTo<Topic>>::scope());
    }
    #[inline]
    fn table() -> String { String::from("categories") }
    #[inline]
    fn id_str() -> String { String::from("category_id") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res =  sqlx::query_as::<Postgres, Self>("
            INSERT INTO categories (id, name, description)
            VALUES ($1, $2, $) RETURNING *")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.description)
            .fetch_one(db).await?;
        Ok(res)
    }
}

#[async_trait::async_trait]
impl LinkedTo<Topic> for Category {
    type LinkModel = TopicCategory;

    /// Served at /catgory/{category_id}/topic
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|id: Path<Id>| respond::ok(format!("GET /category/{}/hi", &id))));
    }
}
