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
pub struct RecordBook {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub record_id: Id,
    #[serde(default = "Id::nil")]
    pub book_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
}
