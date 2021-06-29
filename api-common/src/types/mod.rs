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

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
#[sqlx(type_name = "jwt")]
pub struct JWT(String);

impl Default for JWT {
    fn default() -> Self {
        Self(String::new())
    }
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "feeling", rename_all = "lowercase")]
pub enum Feeling {
    Happy, Sad, Angry, Tired,
}
#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "group_role", rename_all = "lowercase")]
pub enum GroupRole {
    Admin,
    Moderator,
    Member,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Admin,
    SuperUser,
    User,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum Status {
    Active,
    Archived,
    Deleted,
    Deferred,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "status", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
    PreferNotToSay,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "provider_type", rename_all = "lowercase")]
pub enum ProviderType {
    Email,
    Credentials,
    OAuth,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "provider_id", rename_all = "lowercase")]
pub enum Provider {
    Devisa,
    Google,
    GitHub,
    GitLab,
    Facebook,
    LinkedIn,
    Twitter
}
impl Provider {

    pub fn devisa_creds_provider_id() -> Self {
        Self::Devisa
    }

}
impl Default for Provider {
    fn default() -> Self {
        Provider::Devisa
    }
}
impl Default for ProviderType {
    fn default() -> Self {
        ProviderType::Credentials
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::PreferNotToSay
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Active
    }
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}

impl Default for GroupRole {
    fn default() -> Self {
        GroupRole::Member
    }
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

pub fn private() -> bool {
    true
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ApiKeys {
    token: String,
    #[serde(default, skip_serializing_if="Option::is_none")]
    master: Option<String>,
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            &Status::Active => "Active".to_string(),
            &Status::Deferred => "Deferred".to_string(),
            &Status::Deleted => "Deleted".to_string(),
            &Status::Archived => "Archived".to_string(),
        }
    }
}

