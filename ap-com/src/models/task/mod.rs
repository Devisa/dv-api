pub mod book;
pub mod status;
pub mod exec;
pub mod context;
pub mod condition;
pub mod containers;

pub use book::TaskBook;
pub use context::{BookExecutionContext, StepExecutionContext};
pub use condition::TaskConditionType;
pub use status::{TaskStepExecStatus, TaskBookExecStatus};

use actix::prelude::*;
use actix_web::web::ServiceConfig;
use crate::{Id, Db, Model, ModelRoutes};
use derive_more::{AsRef, AsMut, Display, From};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::{PgPool, FromRow, Postgres, types::Json};
use uuid::Uuid;
use crate::{now, private, Status};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    pub name: String,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
#[async_trait::async_trait]
impl Model for Task {
    #[inline]
    fn table() -> String { "tasks".to_string() }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO tasks
            (id, user_id, name, description, status,
            private, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            ")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.status)
            .bind(&self.private)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)

    }
}
#[async_trait::async_trait]
impl ModelRoutes for Task {
    #[inline]
    fn path() -> String { String::from("/task") }

    fn model_routes(cfg: &mut ServiceConfig) {
        cfg;
    }
}



impl Default for Task {
    #[inline]
    fn default() -> Self {
        Self {
            id: Id::gen(),
            name: String::new(),
            description: None,
            user_id: Id::nil(),
            private: false,
            created_at: now(),
            updated_at: now(),
            ..Default::default()
        }
    }
}

impl Task {
    #[inline]
    pub fn new(user_id: Id, name: String) -> Self {
        let id = Uuid::new_v4();
        Self {
            id: Id::new(id),
            name,
            user_id,
            ..Default::default()
        }
    }
}

