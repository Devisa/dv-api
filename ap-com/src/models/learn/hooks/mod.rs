use actix::prelude::*;
use actix_broker::{Broker, BrokerSubscribe,  BrokerMsg, BrokerIssue, SystemBroker};
use std::collections::{BTreeSet as Set, BTreeMap as Map};
use sqlx::types::chrono::NaiveDateTime;
use super::{knowledge::KnowledgeGraph};
use crate::{
    types::*,
    db::Db,
    models::{
        link::{RecordItem, ItemField},
        user::{User, UserData},
        item::{Item, ItemData, ItemRelation, ItemRelationData},
        record::{Record, RecordRelation},
        link::Link,
    },
};
use petgraph::{
    prelude::*,
    graph::{Node, Edge, Graph},
    algo::{ astar, min_spanning_tree }
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LearnUnitWebhook {
    #[serde(default)]
    pub id: Option<i32>,
    pub user_id: i32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub last_triggered: Option<NaiveDateTime>,
    #[serde(default)]
    pub api_keys: Option<ApiKeys>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Actor for LearnUnitWebhook {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Learn unit webhoook from {:?} for URL {:?} has started!", &self.user_id, &self.url);
        // self.subscribe_sync::<Broker, LearnUnitWebhookEvent>(ctx);
        // self.issue_async::<Broker, _>(LearnUnitWebhookEvent { id: None, website_id: 0, created_at: now(), payload: Map::new(), webhook_id: 0, metadata: Map::<String>::new(),  });
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Learn unit webhoook from {:?} for URL {:?} has stopped!", &self.user_id, &self.url);
    }
}

#[derive(Serialize, Deserialize, Debug, Message, Clone)]
#[rtype(result = "()")]
#[serde(untagged)]
pub enum LearnUnitWebhookMessage {
    Initialized,
    UpdatedMetadata(String),
    UpdatedUrl(String),
    FailedRequest { url: String, user_id: i32 },
    Archived,

}

#[derive(Serialize, Deserialize, Debug, Message, Clone)]
#[rtype(result = "()")]
#[serde(deny_unknown_fields)]
pub struct LearnUnitWebhookEvent {
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,
    pub website_id: i32,
    pub webhook_id: i32,
    #[serde(default, skip_serializing_if="Map::is_empty")]
    pub payload: Map<String, String>,
    #[serde(default, skip_serializing_if="Option::is_none")]
    pub metadata: Option<Map<String, String>>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
}

impl Supervised for LearnUnitWebhook {
    fn restarting(&mut self, ctx: &mut <Self as Actor>::Context) {
        println!("Learn unit webhook {:?} now restarting", &self.id);
    }
}

impl Default for LearnUnitWebhook {
    fn default() -> Self {
        Self {
            id: None, created_at: now(), updated_at: now(),
            user_id: 0, url: None, api_keys: None, last_triggered: None,
            description: None, name: None,
        }
    }
}

impl SystemService for LearnUnitWebhook {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        println!("System service for learn unit webhook started");
    }
}

impl Handler<LearnUnitWebhookEvent> for LearnUnitWebhook {
    type Result = ();

    fn handle(&mut self, msg: LearnUnitWebhookEvent, ctx: &mut Self::Context) -> Self::Result {
        println!("Learn Unit Webhook {:?} received new event from {}!", &self.id, &msg.website_id);
        if !msg.payload.is_empty()  {
            LearnUnitWebhookResponse::DoNothing;
        } else {
            LearnUnitWebhookResponse::SendToUser { user_id: self.user_id };
        }

    }
}

// #[derive(MessageResponse, Serialize, Debug, Deserialize, Clone)]
// #[serde(untagged)]
pub enum LearnUnitWebhookResponse {
    AddToRecord {
        record_id: i32,
        link_id: Option<i32>,
        name: Option<String>,
        description: Option<String>,
    },
    ParseText { content: String, context: Option<String> },
    SendToUser { user_id: i32 },
    PerformAction(i32),
    DoNothing,
}

