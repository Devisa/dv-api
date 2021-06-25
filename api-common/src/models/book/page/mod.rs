use actix::prelude::*;
use actix_web::{guard::Guard, HttpRequest, HttpResponse, Responder};
use crate::{
    models::{post::Post, },
    types::{Status, now, private}
};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

/// A page in a book.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Page {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub user_id: i32,
    pub book_id: i32,
    pub content: String,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,

}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PageDiff {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub user_id: i32,
    pub page_id: i32,
    pub content: String,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
}
