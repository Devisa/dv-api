use actix::prelude::*;
use actix_web::{guard::Guard, HttpRequest, HttpResponse, Responder};
use crate::{
    models::{post::Post, },
    types::{Id, Status, now, private}
};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupBook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub group: i32,
    pub book_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<i32>,
}
