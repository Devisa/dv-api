use actix::prelude::*;
use api_db::{Id, Db, Model};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::{PgPool,  FromRow};
use uuid::Uuid;
use crate::types::{ now, private, Status };

#[derive(FromRow, Clone, Serialize, Deserialize, Debug)]
pub struct TaskStep {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(default = "Id::nil")]
    pub task_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_task_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_book_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_index: Option<u16>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,

}

impl Actor for TaskStep {
    type Context = Context<Self>;
    #[inline]
    fn started(&mut self, ctx: &mut Self::Context) {
        tracing::info!("[{}] [TASK_STEP {:?}] has started", self.id, self.name);
    }
    #[inline]
    fn stopped(&mut self, ctx: &mut Self::Context) {
        tracing::info!("[{}] [TASK_STEP {:?}] has stopped", self.id, self.name);
    }
}

impl Default for TaskStep {
    #[inline]
    fn default() -> Self {
        Self {
            id: Id::gen(),
            user_id: Id::nil(),
            task_id: Id::nil(),
            next_task_id: None,
            task_book_id: None,
            step_index: None,
            name: None,
            description: None,
            private: true,
            created_at: now(),
            updated_at: now(),
        }
    }
}

impl TaskStep {
    pub async fn push_to_chain(db: &PgPool, user_id: Id, task_id: Id, prev_task_id: Id) -> Self {
        let id = Uuid::new_v4();
        // TODO check if prev task has task book, if yes, add to this one, else, None
        //      Same for step index
        Self { id: Id::new(id), task_id,  user_id, ..Default::default() }
    }
}
