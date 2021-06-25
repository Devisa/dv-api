use actix::prelude::*;
use api_db::types::Model;
use uuid::Uuid;
use crate::types::now;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

#[async_trait::async_trait]
impl Model for UserBadge {

    fn table() -> String { "user_badges".to_string() }
    fn id_str() -> String { "user_badge_id".to_string() }
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
#[serde(deny_unknown_fields)]
pub struct UserBadge {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil")]
    pub user_level_id: Uuid,
    pub name: String,
    pub description: String,
    pub condition: String,
    #[serde(default = "now")]
    pub achieved_at: NaiveDateTime,

}

impl UserBadge {
    pub fn new(user_level_id: Uuid, name: String, description: String, condition: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_level_id,
            name,
            description,
            achieved_at: now(),
            condition,
        }
    }

}

pub fn starting_level() -> u32 { 0 }
pub fn starting_exp() -> f32 { 0.0 }
