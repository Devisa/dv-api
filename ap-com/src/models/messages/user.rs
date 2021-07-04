use serde::{Serialize, Deserialize};
use crate::now;
use crate::Id;
use crate::models::Model;
use sqlx::{FromRow, Postgres, PgPool, prelude::*, types::{
        chrono::NaiveDateTime,
    }};

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct DirectUserMessage {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub sender_id: Id,
    #[serde(default = "Id::nil")]
    pub recipient_id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub replies_to_id: Option<Id>,
    pub content: String,
    #[serde(default = "Vec::new")]
    pub attachments: Vec<String>,
    #[serde(default = "now")]
    pub sent_at: NaiveDateTime,
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_at: Option<NaiveDateTime>,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for DirectUserMessage {
    fn table() -> String { String::from("direct_group_messages") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO direct_user_messages
            (sender_id, recipient_id, replies_to_id,
             read_at, attachments, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            ")
            .bind(self.sender_id)
            .bind(self.recipient_id)
            .bind(self.replies_to_id)
            .bind(self.read_at)
            .bind(&self.attachments)
            .bind(&self.content)
            .fetch_one(db).await?;
        Ok(res)
    }
}

impl DirectUserMessage {

    pub fn new(
        sender_id: Id,
        recipient_id: Id,
        content: String) -> Self {
        Self {
            id: Id::gen(),
            sender_id,
            recipient_id,
            content,
            attachments: Vec::new(),
            sent_at: now(),
            replies_to_id: None,
            read_at: None,
            updated_at: now(),
        }
    }

    pub async fn reply_to(
        db: &PgPool,
        target_id: Id,
        sender_id: Id,
        recipient_id: Id,
        content: String ) -> anyhow::Result<DirectUserMessage>
    {
        let rs=Self {
            id: Id::gen(),
            sender_id, recipient_id, content,
            replies_to_id: Some(target_id),
            attachments: Vec::new(),
            sent_at: now(),
            read_at: None,
            updated_at: now(),
        }.send(db).await?;
        Ok(rs)
    }

    pub async fn get_all_thread_starters(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE replies_to_id IS NULL
        ")
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_all_thread_starters_with_user(
        db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE replies_to_id IS NULL
            AND user_id = $1
        ")
            .bind(id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_all_replies_with_user(
        db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE replies_to_id IS NULL
            AND user_id = $1
        ")
            .bind(id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_all_replies(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE replies_to_id IS NOT NULL
        ")
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn send(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO direct_user_messages
            (sender_id, recipient_id, replies_to_id,
             read_at, attachments, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            ")
            .bind(self.sender_id)
            .bind(self.recipient_id)
            .bind(self.replies_to_id)
            .bind(self.read_at)
            .bind(&self.attachments)
            .bind(&self.content)
            .fetch_one(db).await?;
        Ok(res)

    }
    pub async fn sent_to_user_id(db: &PgPool, user_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE recipient_id = $1
        ")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn sent_by_user_id(db: &PgPool, user_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE sender_id = $1
        ")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn sent_between_user_ids(db: &PgPool, user_id1: Id, user_id2: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE sender_id = $1, recipient_id = $2
        ")
            .bind(user_id1)
            .bind(user_id2)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_replies_to_dm(db: &PgPool, direct_user_message_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE replies_to_id = $1
        ")
            .bind(direct_user_message_id)
            .fetch_all(db).await?;
        Ok(msg)
    }
}
