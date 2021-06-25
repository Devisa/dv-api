pub mod badge;
pub mod graph;
pub mod level;
pub mod link;
pub mod mail;

use uuid::Uuid;
use actix::prelude::*;
use rand::{distributions::{Uniform, Alphanumeric}, Rng, prelude::Distribution};
use fake::{Dummy, Fake, Faker, faker};
use api_db::types::Model;
use crate::{
    types::{Status, now, private},
    models::{Profile, Item, Record, Account, VerificationRequest, Session, Credentials, Field, Group,
        user::badge::UserBadge,
        messages::{
            user::DirectUserMessage,
            group::DirectGroupMessage,
            topic::DirectTopicMessage,
        }
    },
};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

use self::level::UserLevel;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserId {
    pub user_id: Uuid,
}
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq, )]
pub struct User {
    #[serde(default = "Uuid::new_v4", skip_serializing_if = "Uuid::is_nil")]
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIn {
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<NaiveDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,

}

impl Default for User {
    fn default() -> Self {
        User {
            id: Uuid::new_v4(),
            name: None,
            email: None,
            email_verified: None,
            image: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl From<UserIn> for User {
    fn from(user: UserIn) -> Self {
        User {
            id: Uuid::new_v4(),
            name: Some(user.name),
            image: user.image,
            email: Some(user.email),
            email_verified: user.email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at
        }
    }
}
impl UserIn {

    pub async fn insert(self, db: &PgPool) -> sqlx::Result<User> {
        let user = User::from(self);
        match user.insert(db).await {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }

}

#[async_trait::async_trait]
impl super::Model for User {
    fn table() -> String { String::from("users") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
        INSERT INTO users
        (id, name, email, email_verified, image, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.email)
            .bind(&self.email_verified)
            .bind(&self.image)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)
    }
}

impl User {

    pub fn new(name: Option<String>, email: Option<String>, image: Option<String>) -> Self {
        User {
            name, email, image,
            email_verified: None,
            ..Default::default()
        }
    }

    pub fn gen() -> Self {
        let mut name: String = rand::thread_rng().sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();
        name.push(' ');
        name.push_str(&rand::thread_rng().sample_iter(&Alphanumeric)
            .take(8).map(char::from).collect::<String>());
        let mut email: String = rand::thread_rng().sample_iter(&Alphanumeric)
            .take(7).map(char::from).collect();
        email.push('@');
        email.push_str(&rand::thread_rng().sample_iter(&Alphanumeric)
            .take(8).map(char::from).collect::<String>());
        email.push_str(".com");
        User {
            id: Uuid::new_v4(),
            email: Some(email),
            email_verified: None,
            image: None,
            created_at: now(),
            updated_at: now(),
            name: Some(name),
        }
    }

    // pub async fn clear(self, db: &PgPool) -> sqlx::Result<Vec<Self>> {
    //     let res = sqlx::query_as::<Postgres, Self>("DELETE * FROM users RETURNING *")
    //         .fetch_all(db).await?;
    //     Ok(res)
    // }


    // pub async fn get(db: &PgPool, id: Uuid) -> anyhow::Result<Option<Self>> {
    //     let res = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id = $1")
    //         .bind(id)
    //         .fetch_optional(db).await?;
    //     Ok(res)
    // }

    // pub async fn delete_by_id(db: &PgPool, id: Uuid) -> anyhow::Result<Option<Self>> {
    //     let res = sqlx::query_as::<Postgres, User>("DELETE FROM users WHERE id = $1 RETURNING *")
    //         .bind(id)
    //         .fetch_optional(db).await?;
    //     Ok(res)
    // }

    pub async fn get_by_username(db: &PgPool, username: &str) -> anyhow::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, User>("
            SELECT * FROM users
            INNER JOIN credentials
            ON users.id = credentials.user_id
            WHERE username = $1
            ")
            .bind(username)
            .fetch_optional(db).await?;
        Ok(res)
    }

    pub async fn get_credentials(db: &PgPool, id: Uuid) -> anyhow::Result<Credentials> {
        let res = sqlx::query_as::<Postgres, Credentials>(
            "SELECT * FROM credentials WHERE user_id = $1")
            .bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn get_profile(db: &PgPool, id: Uuid) -> anyhow::Result<Profile> {
        let res = sqlx::query_as::<Postgres, Profile>("
            SELECT * FROM profiles WHERE user_id = $1")
            .bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn get_accounts(db: &PgPool, id: Uuid) -> anyhow::Result<Vec<Account>> {
        let res = sqlx::query_as::<Postgres, Account>("
            SELECT * FROM accounts WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_sessions(db: &PgPool, id: Uuid) -> anyhow::Result<Vec<Session>> {
        let res = sqlx::query_as::<Postgres, Session>("
            SELECT * FROM sessions WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_verification_requests(db: &PgPool, id: Uuid) -> anyhow::Result<Vec<VerificationRequest>> {
        let res = sqlx::query_as::<Postgres, VerificationRequest>("
            SELECT * FROM verification_requests WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_records_created(db: &PgPool, id: Uuid) -> anyhow::Result<Vec<Record>> {
        let res = sqlx::query_as::<Postgres, Record>("
            SELECT * FROM records WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_items_created(db: &PgPool, id: Uuid) -> anyhow::Result<Vec<Item>> {
        let res = sqlx::query_as::<Postgres, Item>("
            SELECT * FROM items WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_fields_created(db: &PgPool, id: Uuid) -> anyhow::Result<Vec<Field>> {
        let res = sqlx::query_as::<Postgres, Field>("
            SELECT * FROM fields WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_groups_created(db: &PgPool, id: Uuid) -> anyhow::Result<Vec<Group>> {
        let res = sqlx::query_as::<Postgres, Group>("
            SELECT * FROM group WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn delete(db: &PgPool, id: Uuid) -> anyhow::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(
            "DELETE FROM users WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    /* pub async fn get_all(db: &PgPool) -> anyhow::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, User>("SELECT * FROM users")
            .fetch_all(db).await?;
        Ok(res)
    } */

    pub async fn add_record(self, db: &PgPool, name: String) -> sqlx::Result<Record> {
        let record = Record::new(name, self.id);
        record.insert(db).await
    }

    pub async fn add_item(self, db: &PgPool, name: String) -> sqlx::Result<Item> {
        let item = Item::new(name, self.id);
        item.insert(db).await
    }

    pub async fn get_level(db: &PgPool, user_id: Uuid) -> anyhow::Result<Option<UserLevel>> {
        let level = UserLevel::get_by_user_id(&db, user_id).await?;
        Ok(level)
    }

    pub async fn get_badges(db: &PgPool, user_id: Uuid) -> anyhow::Result<Vec<UserBadge>> {
        let level = UserLevel::get_by_user_id(&db, user_id).await?;
        if let Some(level) = level {
            let badges = UserLevel::get_badges(&db, level.id).await?;
            Ok(badges)
        } else {
            Ok(vec![])
        }
    }

    // pub async fn get_dms_as_recipient(self, db: &PgPool, user_id: Uuid) -> anyhow::Result<Vec<DirectUserMessage>> {
    //     let level = UserLevel::get_by_user_id(&db, user_id).await?;
    //     if let Some(level) = level {
    //         let badges = UserLevel::get_badges(&db, level.id.unwrap()).await?;
    //         Ok()
    //     } else {
    //         Ok(vec![])
    //     }
    // }

}

pub struct UserCompleteView {
    pub user: User,
    pub credentials: Credentials,
    pub profile: Profile,
    pub accounts: Vec<Account>,
    pub sessions: Vec<Session>,
    pub verification_requests: Vec<VerificationRequest>,
    pub records: Vec<Record>,
    pub items: Vec<Item>,
    pub fields: Vec<Field>,
    pub groups: Vec<Group>,
    pub messages: Vec<DirectUserMessage>
}

impl Actor for User {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("USER: {:?} has entered the network.", &self.email);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("USER: {:?} has left the network.", &self.email);
    }
}

