use super::User;
use actix::prelude::*;
use chrono::NaiveDateTime;
use crate::types::{now, Role, Id, Status, private, };
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRelation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<i32>,
    pub user1_id: i32,
    pub user2_id: i32,
    pub relation: UserRelationKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Message for UserRelation {
    type Result = ();

}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(type_name = "user_relation_kind", rename_all = "lowercase")]
pub enum UserRelationKind {
    Mention,
    DirectMessage,
    PostOnTheirContent,
    ReactToTheirContent,
    Other,
    ProfileView,
}

