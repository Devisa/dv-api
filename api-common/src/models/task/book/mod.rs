pub mod step;

pub use step::TaskStep;
use actix::prelude::*;
use api_db::{Id, Db, Model};
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::{PgPool, FromRow};
use crate::types::{ now, private, Status };

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct TaskBook {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_task_book_id: Option<Id>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl TaskBook {
    pub fn new(id: Id, user_id: Id, name: String, status: Status, private: bool, description: Option<String>, next_task_book_id: Option<Id>, created_at: NaiveDateTime, updated_at: NaiveDateTime) -> Self { Self { id, user_id, name, status, private, description, next_task_book_id, created_at, updated_at } }
}
