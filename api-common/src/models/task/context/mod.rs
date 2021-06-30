use actix::prelude::*;
use api_db::{Id, Db, Model};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::{PgPool, types::Json};
use crate::types::{now, private, Status};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookExecutionContext {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_book_id: Option<Id>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub vars: Vec<ExecutionContextVar>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepExecutionContext {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_step_id: Option<Id>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub vars: Vec<ExecutionContextVar>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContextVar {
    #[serde(default = "Id::gen")]
    pub id: Id,
}
