use actix_web::guard::Post;
use crate::{
    types::{Id, Status, now, private}
};
use sqlx::{FromRow, Postgres, types::chrono::{NaiveDateTime, Utc}, PgPool};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UserBook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub user_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl UserBook {

    #[allow(unused)]
    pub async fn update_page(db: &PgPool) -> () {

    }

    #[allow(unused)]
    pub async fn insert(db: &PgPool) -> () {

    }

    #[allow(unused)]
    pub async fn new_post(db: &PgPool) -> () {

    }

    #[allow(unused)]
    pub async fn add_file(db: &PgPool) -> () {

    }
}
