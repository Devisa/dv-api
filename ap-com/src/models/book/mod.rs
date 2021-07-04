pub mod page;
pub mod post;
pub mod group;
pub mod user;
pub mod topic;
pub mod record;

use uuid::Uuid;
use crate::Model;
use crate::{
    models::post::Post,
    types::{Id, Status, now,Feeling, private}
};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::NaiveDateTime,
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    pub name: String,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "private")]
    pub wiki: bool,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Book {
    fn table() -> String { String::from("tables") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO post_books (id, name, user_id, status,
                image, description, private, wiki, created_at,
                updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,)
            ")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.user_id)
            .bind(&self.status)
            .bind(&self.image)
            .bind(&self.description)
            .bind(&self.private)
            .bind(&self.wiki)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)
    }
}

impl Default for Book {
    fn default() -> Self {
        Self {
            id: Id::gen(),
            user_id: Id::nil(),
            description: None,
            name: String::new(),
            image: None,
            created_at: now(),
            updated_at: now(),
            wiki: true,
            private: true,
            status: Status::default(),
        }
    }
}


impl Book {

    pub fn new(name: String, user_id: Id) -> Self {
        Self { user_id, name, ..Default::default()}
    }

    pub async fn new_post_thread(self,
        db: &PgPool,
        content: String,
        feeling: Option<Feeling>,
        image: Option<String>,
        user_id: Id) -> sqlx::Result<Self>
    {
        let post = Post::new(user_id, content, None, image, feeling)
            .insert(&db)
            .await?;
        let id = sqlx::query_as::<Postgres, Self>("
            INSERT INTO post_books (id, name, user_id, status,
                image, description, private, wiki, created_at,
                updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.user_id)
            .bind(&self.status)
            .bind(&self.image)
            .bind(&self.description)
            .bind(&self.private)
            .bind(&self.wiki)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(id)
    }

    pub async fn add_to_entity(self, db: &PgPool, entity: &str, id: Id, link_id: Option<Id>) -> sqlx::Result<Self> {
        let (table, id_ref) = match entity {
            "group" => ("group_book_links", "group_id"),
            "record" => ("record_book_links", "record_id"),
            "topic" => ("book_topic_links", "topic_id"),
            &_ => ("", "")
        };
        let res = sqlx::query_scalar("INSERT INTO $1 (book_id, $2, link_id)
            VALUES ($3 $4 $5) RETURNING id)")
            .bind(&table)
            .bind(&id_ref)
            .bind(&self.id)
            .bind(&id)
            .bind(&link_id)
            .fetch_one(db).await?;
        Ok(self)

    }
}

