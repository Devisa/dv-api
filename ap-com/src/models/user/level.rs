use uuid::Uuid;
use actix::prelude::*;
use crate::{Model, Id, Db};
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
#[sqlx(rename_all = "snake_case")]
pub struct UserLevel {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
   #[serde(default = "starting_level")]
    pub level: u32,
   #[serde(default = "starting_exp")]
    pub exp: f32,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,

}

#[async_trait::async_trait]
impl Model for UserLevel {
    #[inline]
    fn id_str() -> String { String::from("user_level_id") }
    #[inline]
    fn id(self) -> Id { self.id }
    #[inline]
    fn table() -> String { String::from("user_levels") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("INSERT INTO user_levels
            (id, user_id, level, exp, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.level)
            .bind(&self.exp)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)
    }
}

impl UserLevel {
    #[inline]
    pub fn create(user_id: Id) -> Self {
        Self {
            id: Id::gen(),
            user_id,
            level: starting_level(),
            exp: starting_exp(),
            created_at: now(),
            updated_at: now()
        }
    }

    pub async fn get_by_user_id(db: &PgPool, user_id: Id) -> sqlx::Result<Option<Self>> {
        let level = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM user_levels
            WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(level)
    }

    pub async fn get_badges(db: &PgPool, user_level_id: Id) -> sqlx::Result<Vec<UserBadge>> {
        let badges = sqlx::query_as::<Postgres, UserBadge>("
            SELECT * FROM user_badges
            WHERE user_level_id = $1")
            .bind(user_level_id)
            .fetch_all(db).await?;
        Ok(badges)
    }
}

#[inline]
pub fn starting_level() -> u32 { 0 }
#[inline]
pub fn starting_exp() -> f32 { 0.0 }
