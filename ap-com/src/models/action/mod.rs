use crate::{util::respond, models::Model, types::{Gender, Id, Status, now, private}};
use actix_web::{HttpRequest, HttpResponse, Responder, web::{get, Path, ServiceConfig}};
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Action {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "Status::default")]
    pub status: Status,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Action {
    #[inline]
    fn table() -> String { String::from("actions") }
    #[inline]
    fn path() -> String { String::from("/action") }

    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /action/hi".to_string())));
    }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO actions
            (id, user_id, name, description, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            ")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.status)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)
    }

}

impl Action {


}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActionLink {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub action1_id: Id,
    #[serde(default = "Id::nil")]
    pub action2_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<Id>,
    pub rel: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
