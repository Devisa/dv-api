use actix::prelude::*;
use actix_web::{guard::Post, Responder};
use crate::types::{Status, now, private};
use sqlx::{FromRow, Postgres, postgres::PgPool, types::chrono::{NaiveDateTime, Utc}};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerificationRequest {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    pub identifier: String,
    pub token: String,
    pub expires: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for VerificationRequest {
    fn default() -> Self {
        VerificationRequest {
            id: Uuid::new_v4(),
            identifier: String::new(),
            token: String::new(),
            expires: Utc::now().naive_utc(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[async_trait::async_trait]
impl super::Model for VerificationRequest {
    fn table() -> String { String::from("verification_requests") }
    async fn insert(self, db: &PgPool,) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO verification_requests (id, identifier, token, expires)
            VALUES ($1, $2, $3, $4) RETURNING *")
            .bind(self.id)
            .bind(self.identifier)
            .bind(self.token)
            .bind(self.expires)
            .fetch_one(db)
            .await?;
        Ok(res)
    }
}

impl VerificationRequest {

    /* pub async fn get_all(db: &PgPool) -> anyhow::Result<Vec<Self>> {
        let ver = sqlx::query_as::<Postgres, Self>("SELECT * FROM verification_requests")
            .fetch_all(db).await?;
        Ok(ver)
    }

    pub async fn get_by_user_id(db: &PgPool, user_id: Uuid) -> anyhow::Result<Option<Self>> {
        let ver = sqlx::query_as::<Postgres, Self>("SELECT * FROM verification_requests WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(ver)
    }

    pub async fn delete_by_user_id(db: &PgPool, user_id: Uuid) -> anyhow::Result<Option<Uuid>> {
        let ver = sqlx::query_scalar("DELETE FROM verification_requests WHERE user_id = $1 returning id")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(ver)
    }

    pub async fn delete_by_id(db: &PgPool, id: Uuid) -> anyhow::Result<Option<Uuid>> {
        let ver = sqlx::query_scalar("DELETE FROM verification_requests WHERE id = $1 returning id")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(ver)
    } */

}
