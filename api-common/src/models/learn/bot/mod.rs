use sqlx::types::chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use actix::prelude::*;
use super::scrape::Scraper;
use crate::{
    types::*
};

/// The user interface to their record(s) learning
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearnUnitBot {
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(default)]
    pub user_id: Option<i32>,
    #[serde(default)]
    pub record_id: Option<i32>,
    #[serde(default = "now")]
    pub last_update_id: NaiveDateTime,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Default for LearnUnitBot {
    fn default() -> Self {
        Self {
            id: None,
            user_id: None,
            record_id: None,
            last_update_id: now(),
            created_at: now(),
            updated_at: now(),
        }
    }
}

impl Supervised for LearnUnitBot {}

impl SystemService for LearnUnitBot {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("Telegram bot service started");
    }
}

impl Actor for LearnUnitBot {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

    }
}
