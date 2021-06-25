use uuid::Uuid;
use actix::prelude::*;
use crate::{
    types::now,
    models::user::UserBadge,
};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::NaiveDateTime,
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UserLevel {
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if = "Uuid::is_nil")]
    pub user_id: Uuid,
   #[serde(default = "starting_level")]
    pub level: u32,
   #[serde(default = "starting_exp")]
    pub exp: f32,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,

}

impl UserLevel {
    pub fn create(user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            level: starting_level(),
            exp: starting_exp(),
            created_at: now(),
            updated_at: now()
        }
    }

    pub async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let id = sqlx::query_as::<Postgres, Self>("INSERT INTO user_levels
            (id, user_id, level, exp, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.level)
            .bind(&self.exp)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(self)
    }

    pub async fn get_by_user_id(db: &PgPool, user_id: uuid::Uuid) -> sqlx::Result<Option<Self>> {
        let level = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM user_levels
            WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(level)
    }

    pub async fn get_badges(db: &PgPool, user_level_id: Uuid) -> sqlx::Result<Vec<UserBadge>> {
        let badges = sqlx::query_as::<Postgres, UserBadge>("
            SELECT * FROM user_badges
            WHERE user_level_id = $1")
            .bind(user_level_id)
            .fetch_all(db).await?;
        Ok(badges)
    }
}

pub fn starting_level() -> u32 { 0 }
pub fn starting_exp() -> f32 { 0.0 }
