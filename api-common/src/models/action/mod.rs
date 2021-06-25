use crate::{models::{
    Model,
        record::Record,
        item::Item,
    }, types::{Gender, Status, now, private}};
use actix_web::{HttpRequest, HttpResponse, Responder};
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub user_id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: Status,
    #[serde(default = "now")]
    pub created: NaiveDateTime,
    #[serde(default = "now")]
    pub modified: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Action {
    fn table() -> String { String::from("actions") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO actions (user_id, name, description, status)
            VALUES ($1, $2, $3, $4) RETURNING *
            ")
            .bind(&self.user_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.status)
            .fetch_one(db).await?;
        Ok(res)
    }
}

impl Action {

}

pub struct ActionLink {
    pub id: Option<i32>,
    pub action1_id: i32,
    pub action2_id: i32,
    pub rel: String,
    pub description: Option<String>
}
