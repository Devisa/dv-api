pub mod id;
pub mod token;

use uuid::Uuid;
use chrono::{Duration, NaiveDateTime, Utc};
use crate::models::{record::Record, user::{UserIn, User}};
use serde::{Deserialize, Serialize};
use std::string::ToString;

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
#[sqlx(type_name = "link_id")]
pub struct LinkId(Option<Uuid>);

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
#[sqlx(type_name = "user_id")]
pub struct UserId(uuid::Uuid);

impl Default for UserId {

    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}
impl From<User> for UserId {
    fn from(us: User) -> Self {
        Self(us.id)
    }
}
impl From<Record> for UserId {
    fn from(rec: Record) -> Self {
        Self(rec.user_id)
    }
}

impl UserId {

    pub fn gen() -> Self {
        Self::default()
    }

    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
#[sqlx(type_name = "jwt")]
pub struct JWT(String);

impl Default for JWT {
    fn default() -> Self {
        Self(String::new())
    }
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(type_name = "expiration", rename_all = "lowercase")]
pub enum Expiration {
    OneDay,
    TwoDays,
    OneWeek,
    TwoWeeks,
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

impl Default for Expiration {
    fn default() -> Self {
        Expiration::OneWeek
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

impl Expiration {

    pub fn two_days() -> NaiveDateTime {
        let today = Utc::now().naive_utc();
        let two_days = today.checked_add_signed(Duration::days(2))
            .expect("Invalid datetime?");
        return two_days;
    }
}
