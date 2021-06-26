use serde::{Serialize, Deserialize};
use crate::types::{GroupRole, now};
use api_db::types::{Id, Model};
use sqlx::{
    prelude::*, PgPool, Postgres,
    FromRow, types::chrono::NaiveDateTime,
};

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct DirectGroupMessage {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub sender_id: Id,
    #[serde(default = "Id::nil")]
    pub group_id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub replies_to_id: Option<Id>,
    pub content: String,
    #[serde(default = "Vec::new")]
    pub attachments: Vec<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub restrict_to_role: Option<GroupRole>,
    #[serde(skip_serializing, default = "now")]
    pub sent_at: NaiveDateTime,
    #[serde(skip_serializing, default = "now")]
    pub updated_at: NaiveDateTime,

}

#[async_trait::async_trait]
impl Model for DirectGroupMessage {
    fn table() -> String { String::from("direct_group_messages") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO direct_group_messages
            (sender_id, group_id, replies_to_id,
             sent_at, restrict_to_role, attachments, content)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            ")
            .bind(self.sender_id)
            .bind(self.group_id)
            .bind(self.replies_to_id)
            .bind(self.sent_at)
            .bind(self.restrict_to_role.as_ref())
            .bind(&self.attachments)
            .bind(&self.content)
            .fetch_one(db).await?;
        Ok(res)
    }
}

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct DirectGroupMessageReadReceipt {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub direct_group_msg_id: Id,
    #[serde(default = "Id::nil")]
    pub group_id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub read_at: Option<NaiveDateTime>,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl DirectGroupMessage {

    pub fn new(
        sender_id: Id,
        group_id: Id,
        content: String) -> DirectGroupMessage {
        let msg = Self {
            id: Id::gen(),
            restrict_to_role: None,
            sender_id,
            group_id,
            content,
            attachments: Vec::new(),
            sent_at: now(),
            replies_to_id: None,
            updated_at: now(),
        };
        return msg;
    }

    pub async fn reply_to(
        db: &PgPool,
        replies_to: Id,
        sender_id: Id,
        group_id: Id,
        content: String
        ) -> anyhow::Result<Self>
    {
        let res =  Self {
            id: Id::gen(),
            replies_to_id: Some(replies_to),
            group_id, sender_id, content,
            updated_at: now(),
            sent_at: now(),
            restrict_to_role: None,
            attachments: Vec::new()
        }
        .send(db).await?;
        Ok(res)
    }

    pub async fn get_all_thread_starters(db: &PgPool
        ) -> sqlx::Result<Vec<Self>>
    {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_group_messages
            WHERE replies_to_id IS NULL
        ")
            .fetch_all(db).await?;
        Ok(msg)
    }
    pub async fn get_all_non_thread_starters(db: &PgPool)
        -> sqlx::Result<Vec<Self>>
    {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_group_messages
            WHERE replies_to_id IS NOT NULL
        ")
            .fetch_all(db).await?;
        Ok(msg)
    }
    pub async fn get_all_thread_starters_with_group(
        db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE replies_to_id IS NULL
            AND group_id = $1
        ")
            .bind(id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_all_replies_with_group(
        db: &PgPool, id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_user_messages
            WHERE replies_to_id IS NULL
            AND group_id = $1
        ")
            .bind(id)
            .fetch_all(db).await?;
        Ok(msg)
    }
    pub async fn send(&self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO direct_group_messages
            (id, sender_id, group_id, replies_to_id,
             sent_at, restrict_to_role, attachments, content)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            ")
            .bind(&self.id)
            .bind(&self.sender_id)
            .bind(&self.group_id)
            .bind(&self.replies_to_id)
            .bind(self.sent_at)
            .bind(self.restrict_to_role.as_ref())
            .bind(&self.attachments)
            .bind(&self.content)
            .fetch_one(db).await?;
        Ok(res)

    }
    pub async fn sent_to_group_id(db: &PgPool, group_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_group_messages
            WHERE group_id = $1
        ")
            .bind(group_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn sent_by_sender_id(db: &PgPool, user_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_group_messages
            WHERE sender_id = $1
        ")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn all_from_sender_to_group(db: &PgPool, user_id: Id, group_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_group_messages
            WHERE sender_id = $1, group_id = $2
        ")
            .bind(user_id)
            .bind(group_id)
            .fetch_all(db).await?;
        Ok(msg)
    }

    pub async fn get_replies_to_dm(db: &PgPool, direct_group_message_id: Id) -> sqlx::Result<Vec<Self>> {
        let msg = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM direct_group_messages
            WHERE replies_to_id = $1
        ")
            .bind(direct_group_message_id)
            .fetch_all(db).await?;
        Ok(msg)
    }
}
