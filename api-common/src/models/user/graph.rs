
use super::{User, link::UserRelation};
use actix::prelude::*;
use chrono::NaiveDateTime;
use crate::types::{now, Role, Status, private, };
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGraph {
    pub rels: Vec<UserRelation>,
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}
impl Actor for UserGraph {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("USER KNOWLEDGE GRAPH: Started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        log::info!("USER KNOWLEDGE GRAPH: Stopped");
    }
}
impl Handler<UserRelation> for UserGraph {
    type Result = ();

    fn handle(&mut self, msg: UserRelation, ctx: &mut Context<Self>) -> Self::Result {
        log::info!("New user relation received: {:?}", msg);
        self.rels.push(msg);
    }
}

impl Supervised for UserGraph {}
