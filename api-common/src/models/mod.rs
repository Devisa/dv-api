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
pub mod item;
pub mod field;
pub mod record;

use chrono::NaiveDateTime;
pub use user::{User,
    session::Session,
    credentials::Credentials,
    verification::VerificationRequest,
    profile::Profile,
    account::Account
};
pub use record::{RecordRelation, Record};
pub use link::Link;
pub use item::{Item, ItemRelation};
pub use field::{FieldRelation, Field};
pub use post::Post;
pub use group::Group;
pub use topic::Topic;
pub use action::Action;
pub use automata::Automata;
pub use messages::{DirectUserMessage, DirectGroupMessage, DirectTopicMessage, DirectGroupMessageReadReceipt};
// pub use learn::LearningUnit;
// pub use book::{UserBook, RecordBook, GroupBook, TopicBook};
// pub use condition::Condition;

pub use api_db::types::Model;


use sqlx::{FromRow, PgPool, Postgres, postgres::PgRow};

pub async fn get<'r, F: 'r + FromRow<'r, PgRow>>() {

}

pub async fn get_all<'r, F: 'r + FromRow<'r, PgRow>>() {

}

pub async fn post<'r, F: 'r + FromRow<'r, PgRow>>() {

}
