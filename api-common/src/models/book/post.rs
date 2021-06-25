use actix::prelude::*;
use uuid::Uuid;
use api_db::types::Model;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct BookPost {
    #[serde(default = "Uuid::new_v4", skip_serializing_if="Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub post_id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub book_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Uuid>,
}

#[async_trait::async_trait]
impl Model for BookPost {
    fn table() -> String {
        "book_posts".to_string()
    }
    fn id_str() -> String {
        "book_post_id".to_string()
    }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("INSERT INTO post_book_entries
            (id, post_id, book_id, link_id) VALUES ($1 $2 $3) RETURNING id")
            .bind(&self.id)
            .bind(&self.post_id)
            .bind(&self.book_id)
            .bind(&self.link_id)
            .fetch_one(db).await?;
        Ok(res)
    }
}

impl BookPost {

    pub fn new(post_id: Uuid, book_id: Uuid, link_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            post_id, book_id, link_id
        }
    }
}
