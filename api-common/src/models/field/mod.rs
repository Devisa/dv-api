pub mod value;
pub mod target;

use uuid::Uuid;
use actix::prelude::*;
use crate::types::{Id, Status, now, private};
use serde::{Serialize, Deserialize};
use crate::models::Model;
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Field {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "private")]
    pub private: bool,
    #[serde(default = "FieldKind::default")]
    pub kind: FieldKind,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Field {

    fn table() -> String { String::from("fields") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO fields (id, name, user_id, kind, private, status, description, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id
            ")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.user_id)
            .bind(&self.kind)
            .bind(&self.private)
            .bind(&self.status)
            .bind(&self.description)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)
    }
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldRelation {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    #[serde(default = "Id::nil")]
    pub record1_id: Id,
    #[serde(default = "Id::nil")]
    pub record2_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            id: Id::gen(),
            user_id: Id::nil(),
            name: String::new(),
            private: true,
            kind: FieldKind::default(),
            status: Status::Active,
            description: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
impl Actor for Field {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("FIELD ACTOR STARTED: {:?}", self.id);
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        log::info!("FIELD ACTOR STOPPED: {:?}", self.id);
    }
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(rename = "field_kind", rename_all="lowercase")]
pub enum FieldKind {
    Integer,
    RealNum,
    Double,
    Range,
    Date,
    DateTime,
    Enumeration,
    Selection,
    Text,
    Boolean
}

impl Default for FieldKind {
    fn default() -> Self {
        FieldKind::Text
    }
}

impl Field {

    pub fn new(name: String, kind: FieldKind, user_id: Id) -> Self {
        Self { name, kind, user_id, ..Default::default() }
    }

}
