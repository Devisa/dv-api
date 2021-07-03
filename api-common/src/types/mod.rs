pub mod time;
pub mod auth;
pub mod token;

use derive_more::{FromStr, Display};
pub use time::Expiration;
pub use api_db::types::id::Id;
pub use token::{AccessToken, SessionToken, RefreshToken};
use uuid::Uuid;
use chrono::{Duration, NaiveDateTime, Utc};
use crate::models::{record::Record, user::{UserIn, User}};
use serde::{Deserialize, Serialize};
use std::string::ToString;


#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "feeling", rename_all = "lowercase")]
pub enum Feeling {
    Happy, Sad, Angry, Tired,
}
#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "group_role", rename_all = "lowercase")]
pub enum GroupRole {
    Admin, Moderator, Member,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Admin, SuperUser, User,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum Status {
    Active, Archived, Deleted, Deferred,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum Gender {
    Male, Female, Other, PreferNotToSay,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ApiKeys {
    token: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    master: Option<String>,
}

impl Default for Gender {
    #[inline]
    fn default() -> Self {
        Gender::PreferNotToSay
    }
}

impl Default for Status {
    #[inline]
    fn default() -> Self {
        Status::Active
    }
}

impl Default for Role {
    #[inline]
    fn default() -> Self {
        Role::User
    }
}

impl Default for GroupRole {
    #[inline]
    fn default() -> Self {
        GroupRole::Member
    }
}

#[inline]
pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

#[inline]
pub fn private() -> bool {
    true
}



impl ToString for Status {
    #[inline]
    fn to_string(&self) -> String {
        match self {
            &Status::Active => "Active".to_string(),
            &Status::Deferred => "Deferred".to_string(),
            &Status::Deleted => "Deleted".to_string(),
            &Status::Archived => "Archived".to_string(),
        }
    }
}

