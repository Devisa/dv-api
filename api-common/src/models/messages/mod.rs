pub mod user;
pub mod group;
pub mod topic;
pub use user::DirectUserMessage;
use uuid::Uuid;
pub use group::{DirectGroupMessageReadReceipt, DirectGroupMessage};
pub use topic::DirectTopicMessage;
use chrono::NaiveDateTime;
use crate::types::now;
use sqlx::PgPool;

pub struct GenericDirectMessage {

}

// impl GenericDirectMessage {
//     pub async fn get_all(db: &PgPool) -> sqlx::Result<Vec
// }

#[derive(Default)]
pub struct GenericMessageBuilder {
    pub id: Uuid,
    pub sender_id: Option<Uuid>,
    pub recipient_id: Option<Uuid>,
    pub replies_to_id: Option<Uuid>,
    pub content: Option<String>,
    pub attachments: Vec<String>,
    pub sent_at: Option<NaiveDateTime>,
    pub read_at: Option<NaiveDateTime>,
}

impl GenericMessageBuilder {

    pub fn new(
        sender_id: Option<Uuid>,
        recipient_id: Option<Uuid>,
        replies_to_id: Option<Uuid>,
        content: Option<String>,
        attachments: Vec<String>,
        sent_at: Option<NaiveDateTime>,
        read_at: Option<NaiveDateTime>
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender_id, recipient_id, replies_to_id, content, attachments, sent_at, read_at
        }
    }

    pub async fn send_to_user(self,
        db: &PgPool,
        user_id: Uuid) -> anyhow::Result<DirectUserMessage> {
        let user_msg = DirectUserMessage {
            recipient_id: user_id,
            id: self.id,
            content: self.content.unwrap_or_default(),
            sent_at: self.sent_at.unwrap_or(now()),
            read_at: self.read_at,
            attachments: self.attachments,
            replies_to_id: self.replies_to_id,
            sender_id: self.sender_id.unwrap_or_default(),
            updated_at: now()
        };
        let msg = user_msg.send(db).await?;
        Ok(msg)
    }

    pub async fn send_to_topic(self,
        db: &PgPool,
        topic_id: Uuid) -> anyhow::Result<DirectTopicMessage> {
        let topic_msg = DirectTopicMessage {
            topic_id,
            id: self.id,
            content: self.content.unwrap_or_default(),
            sent_at: self.sent_at.unwrap_or(now()),
            attachments: self.attachments,
            replies_to_id: self.replies_to_id,
            sender_id: self.sender_id.unwrap_or_default(),
            updated_at: now()
        };
        let msg = topic_msg.send(db).await?;
        Ok(msg)
    }

    pub async fn send_to_group(self,
        db: &PgPool,
        group_id: Uuid
    ) -> anyhow::Result<DirectGroupMessage>
    {
        let group_msg = DirectGroupMessage {
            group_id,
            id: self.id,
            content: self.content.unwrap_or_default(),
            sent_at: self.sent_at.unwrap_or(now()),
            attachments: self.attachments,
            replies_to_id: self.replies_to_id,
            sender_id: self.sender_id.unwrap_or_default(),
            restrict_to_role: None,
            updated_at: now()
        };
        let msg = group_msg.send(db).await?;
        Ok(msg)
    }
}
