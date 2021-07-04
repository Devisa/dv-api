use crate::Id;
use super::GenericMessageBuilder;
use serde::{Serialize, Deserialize};
use crate::{
    types::now,
    models::Model,
};
use sqlx::{
    prelude::*, Postgres, PgPool,
    FromRow, types::chrono::NaiveDateTime,
};
use chrono::Utc;

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct DirectTopicMessage {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub sender_id: Id,
    #[serde(default = "Id::nil")]
    pub topic_id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub replies_to_id: Option<Id>,
    pub content: String,
    #[serde(default = "Vec::new")]
    pub attachments: Vec<String>,
    #[serde(default = "now")]
    pub sent_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for DirectTopicMessage {
    fn table() -> String { String::from("direct_topic_messages") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO direct_topic_messages
            (id, sender_id, topic_id, replies_to_id, content,
             attachments)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            ")
            .bind(self.id)
            .bind(self.sender_id)
            .bind(self.topic_id)
            .bind(self.replies_to_id)
            .bind(self.content.as_str())
            .bind(&self.attachments)
            .fetch_one(db).await?;
        Ok(res)
    }
}

impl Default for DirectTopicMessage {
    fn default() -> Self {
        Self {
            id: Id::gen(),
            sender_id: Id::nil(),
            topic_id: Id::nil(),
            replies_to_id: None,
            content: String::new(),
            attachments: Vec::new(),
            sent_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        }
    }
}

impl DirectTopicMessage {

    pub fn new(
        sender_id: Id,
        topic_id: Id,
        content: String) -> GenericMessageBuilder {
        let msg = GenericMessageBuilder {
            sender_id: Some(sender_id),
            recipient_id: Some(topic_id),
            content: Some(content),
            ..Default::default()
        };
        return msg;
    }

    pub async fn new_thread(
        db: &PgPool,
        sender_id: Id,
        topic_id: Id,
        content: String) -> anyhow::Result<DirectTopicMessage>
    {
        GenericMessageBuilder {
            sender_id: Some(sender_id),
            recipient_id: Some(topic_id.clone()),
            content: Some(content),
            ..Default::default()
        }.send_to_topic(db, topic_id).await
    }

    pub async fn reply_to(
        db: &PgPool,
        target_id: Id,
        sender_id: Id,
        topic_id: Id,
        content: String ) -> anyhow::Result<Self>
    {
        let rs = Self {
            id: Id::gen(),
            sender_id, topic_id, content,
            replies_to_id: Some(target_id),
            ..Default::default()
        }
        .send(db).await?;
        Ok(rs)
    }

    pub async fn send(&self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO direct_topic_messages
            (id, sender_id, topic_id, replies_to_id, content,
             attachments)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            ")
            .bind(&self.id)
            .bind(&self.sender_id)
            .bind(&self.topic_id)
            .bind(&self.replies_to_id)
            .bind(self.content.as_str())
            .bind(&self.attachments)
            .fetch_one(db).await?;
        Ok(res)

    }
    pub async fn get_all_topic(db: &PgPool, topic_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE topic_id = $1
        ")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(msg)
    }
    pub async fn get_all_thread_starters(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE replies_to_id IS NULL
        ")
            .fetch_all(db).await?;
        Ok(msg)
    }
    pub async fn get_all_replies(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE replies_to_id IS NOT NULL
        ")
            .fetch_all(db).await?;
        Ok(msg)
    }
    pub async fn get_all_topic_thread_starters(
        db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE replies_to_id IS NULL
            AND topic_id = $1
        ")
            .bind(id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_all_topic_replies(
        db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE replies_to_id IS NULL
            AND topic_id = $1
        ")
            .bind(id)
            .fetch_all(db).await?;
        Ok(msg)
    }
    pub async fn sent_to_topic_id(db: &PgPool, topic_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE recipient_id = $1
        ")
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn sent_by_sender_id(db: &PgPool, user_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE sender_id = $1
        ")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn all_from_sender_to_topic(db: &PgPool, user_id: Id, topic_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE sender_id = $1, topic_id = $2
        ")
            .bind(user_id)
            .bind(topic_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_replies_to_dm(db: &PgPool, direct_topic_message_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_topic_messages
            WHERE replies_to_id = $1
        ")
            .bind(direct_topic_message_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_thread_starters(db: &PgPool, user_id: Option<Id>, topic_id: Option<Id>) -> sqlx::Result<Vec<Self>> {
        let msgs = match (&user_id, &topic_id) {
            (Some(user_id), Some(topic_id)) => sqlx::query_as::<Postgres, Self>("
                SELECT * FROM direct_topic_messages
                WHERE replies_to_id = null
                    AND user_id = $1
                    AND topic_id = $2")
                .bind(user_id)
                .bind(topic_id)
                .fetch_all(db).await?,
            (Some(user_id), None) => sqlx::query_as::<Postgres, Self>("
                SELECT * FROM direct_topic_messages
                WHERE replies_to_id = null
                    AND user_id = $1")
                .bind(user_id)
                .fetch_all(db).await?,
            (None, Some(user_id)) => sqlx::query_as::<Postgres, Self>("
                SELECT * FROM direct_topic_messages
                WHERE replies_to_id = null
                    AND topic_id = $1")
                .bind(topic_id)
                .fetch_all(db).await?,
            (None, None) => sqlx::query_as::<Postgres, Self>("
                SELECT * FROM direct_topic_messages
                WHERE replies_to_id = null")
                .fetch_all(db).await?
        };
        Ok(msgs)
    }
}
