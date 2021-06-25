pub mod auth;
pub mod culture;
pub mod feel;
pub mod action;
pub mod automata;
pub mod ai;
pub mod messages;
pub mod condition;
pub mod channel;
pub mod topic;
pub mod book;
pub mod link;
pub mod user;
pub mod group;
pub mod post;
pub mod task;
pub mod account;
pub mod session;
pub mod verification;
pub mod profile;
pub mod credentials;
pub mod item;
pub mod field;
pub mod record;
// pub mod learn;

use chrono::NaiveDateTime;
pub use record::{RecordRelation, Record};
pub use link::Link;
pub use user::User;
pub use profile::Profile;
pub use item::{Item, ItemRelation};
pub use field::{FieldRelation, Field};
pub use credentials::Credentials;
pub use verification::VerificationRequest;
pub use session::Session;
pub use post::Post;
pub use group::Group;
pub use topic::Topic;
pub use account::Account;
// pub use learn::LearningUnit;
// pub use book::{UserBook, RecordBook, GroupBook, TopicBook};
pub use action::Action;
pub use automata::Automata;
// pub use condition::Condition;
pub use messages::{DirectUserMessage, DirectGroupMessage, DirectTopicMessage, DirectGroupMessageReadReceipt};

pub use api_db::types::Model;


use sqlx::{FromRow, PgPool, Postgres, postgres::PgRow};

pub async fn get<'r, F: 'r + FromRow<'r, PgRow>>() {

}

pub async fn get_all<'r, F: 'r + FromRow<'r, PgRow>>() {

}

pub async fn post<'r, F: 'r + FromRow<'r, PgRow>>() {

}
