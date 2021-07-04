use actix::prelude::*;
use crate::Model;
use crate::{now, Id};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

#[async_trait::async_trait]
impl Model for UserBadge {
    #[inline]
    fn table() -> String { "user_badges".to_string() }
    #[inline]
    fn id_str() -> String { "user_badge_id".to_string() }
    #[inline]
    fn id(self) -> Id { self.id }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("INSERT INTO user_badges
            (id, user_level_id, name, description, condition, achieved_at)
            VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&self.id)
            .bind(&self.user_level_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(&self.condition)
            .bind(&self.achieved_at)
            .fetch_one(db).await?;
        Ok(res)
    }
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(rename_all = "snake_case")]
#[serde(deny_unknown_fields)]
pub struct UserBadge {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_level_id: Id,
    pub name: String,
    pub description: String,
    pub condition: String,
    #[serde(default = "now")]
    pub achieved_at: NaiveDateTime,

}

impl UserBadge {
    #[inline]
    pub fn new(user_level_id: Id, name: String, description: String, condition: String) -> Self {
        Self {
            id: Id::gen(),
            user_level_id,
            name,
            description,
            achieved_at: now(),
            condition,
        }
    }
}

#[inline]
pub fn starting_level() -> u32 { 0 }
#[inline]
pub fn starting_exp() -> f32 { 0.0 }
