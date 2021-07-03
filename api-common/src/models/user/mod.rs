pub mod badge;
pub mod feel;
pub mod verification;
pub mod graph;
pub mod level;
pub mod session;
pub mod credentials;
pub mod link;
pub mod mail;
pub mod account;
pub mod profile;

use std::borrow::Cow;
use serde::{Serialize, Deserialize};
use actix::prelude::*;
use rand::{distributions::{Uniform, Alphanumeric}, Rng, prelude::Distribution};
use api_db::types::Model;
use crate::{
    types::{Id, Status, now, private},
    models::{Profile, Item, Record, Account, VerificationRequest, Session, Credentials, Field, Group,
        user::badge::UserBadge,
        messages::{
            user::DirectUserMessage,
            group::DirectGroupMessage,
            topic::DirectTopicMessage,
        }
    },
};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

use self::level::UserLevel;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq, )]
#[sqlx(rename_all = "snake_case")]
pub struct User {
    #[serde(default = "Id::gen")]
    pub id: Id,
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
    #[serde(default = "String::new")]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    #[inline]
    fn default() -> Self {
        User {
            id: Id::gen(),
            name: None,
            email: None,
            email_verified: None,
            image: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl Default for UserIn {
    #[inline]
    fn default() -> Self {
        UserIn{
            name: None,
            image: None,
            email: String::new(),
            updated_at: now(),
            created_at: now(),
            email_verified: None,
        }
    }
}

impl From<UserIn> for User {
    #[inline]
    fn from(user: UserIn) -> Self {
        User {
            id: Id::gen(),
            name: user.name,
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
impl Model for User {
    #[inline]
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

    #[inline]
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
            id: Id::gen(),
            email: Some(email),
            email_verified: None,
            image: None,
            created_at: now(),
            updated_at: now(),
            name: Some(name),
        }
    }

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

    pub async fn get_credentials(db: &PgPool, id: Id) -> anyhow::Result<Credentials> {
        let res = sqlx::query_as::<Postgres, Credentials>(
            "SELECT * FROM credentials WHERE user_id = $1")
            .bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn get_profile(db: &PgPool, id: Id) -> anyhow::Result<Profile> {
        let res = sqlx::query_as::<Postgres, Profile>("
            SELECT * FROM profiles WHERE user_id = $1")
            .bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn get_accounts(db: &PgPool, id: Id) -> anyhow::Result<Vec<Account>> {
        let res = sqlx::query_as::<Postgres, Account>("
            SELECT * FROM accounts WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_sessions(db: &PgPool, id: Id) -> anyhow::Result<Vec<Session>> {
        let res = sqlx::query_as::<Postgres, Session>("
            SELECT * FROM sessions WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_verification_requests(db: &PgPool, id: Id) -> anyhow::Result<Vec<VerificationRequest>> {
        let res = sqlx::query_as::<Postgres, VerificationRequest>("
            SELECT * FROM verification_requests WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_records_created(db: &PgPool, id: Id) -> anyhow::Result<Vec<Record>> {
        let res = sqlx::query_as::<Postgres, Record>("
            SELECT * FROM records WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_items_created(db: &PgPool, id: Id) -> anyhow::Result<Vec<Item>> {
        let res = sqlx::query_as::<Postgres, Item>("
            SELECT * FROM items WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_fields_created(db: &PgPool, id: Id) -> anyhow::Result<Vec<Field>> {
        let res = sqlx::query_as::<Postgres, Field>("
            SELECT * FROM fields WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get_groups_created(db: &PgPool, id: Id) -> anyhow::Result<Vec<Group>> {
        let res = sqlx::query_as::<Postgres, Group>("
            SELECT * FROM group WHERE user_id = $1")
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn delete(db: &PgPool, id: Id) -> anyhow::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(
            "DELETE FROM users WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    pub async fn add_record(self, db: &PgPool, name: String) -> sqlx::Result<Record> {
        let record = Record::new(name, self.id);
        record.insert(db).await
    }

    pub async fn add_item(self, db: &PgPool, name: String) -> sqlx::Result<Item> {
        let item = Item::new(name, self.id);
        item.insert(db).await
    }

    pub async fn get_level(db: &PgPool, user_id: Id) -> anyhow::Result<Option<UserLevel>> {
        let level = UserLevel::get_by_user_id(&db, user_id).await?;
        Ok(level)
    }

    pub async fn get_badges(db: &PgPool, user_id: Id) -> anyhow::Result<Vec<UserBadge>> {
        let level = UserLevel::get_by_user_id(&db, user_id).await?;
        if let Some(level) = level {
            let badges = UserLevel::get_badges(&db, level.id).await?;
            Ok(badges)
        } else {
            Ok(vec![])
        }
    }

    // pub async fn get_dms_as_recipient(self, db: &PgPool, user_id: Id) -> anyhow::Result<Vec<DirectUserMessage>> {
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

    #[inline]
    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("USER: {:?} has entered the network.", &self.email);
    }

    #[inline]
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("USER: {:?} has left the network.", &self.email);
    }
}

impl async_graphql::Type for User {
    #[inline]
    fn type_name() -> std::borrow::Cow<'static, str> {
        Cow::Owned("user".to_string())
    }
    #[inline]
    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> String {
        "user".to_string()
    }
    #[inline]
    fn qualified_type_name() -> String {
        "user".to_string()
    }
    #[inline]
    fn introspection_type_name(&self) -> std::borrow::Cow<'static, str> {
        Cow::Owned("user".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdentifierQuery {
    id: Option<Id>,
    username: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFilterQuery {

}


#[cfg(test)]
mod tests {
    use api_db::{Db, Model};
    use super::*;

    fn user(name: &str, email: &str, image: Option<&str>) -> User {
        User {
            name: Some(name.to_string()),
            email: Some(email.to_string()),
            image: image.map(|s| s.to_string()),
            ..Default::default()
        }
    }

    async fn db() -> anyhow::Result<Db> {
        Db::new(&dotenv::var("DATABASE_URL").unwrap()).await
    }

    // NOTE this function can/should be generalized among any models
    //      using type params
    #[actix::test]
    async fn inserts_retrieves_user_ok() -> sqlx::Result<()> {
        let db = db().await.unwrap();
        let user_in: User = user("user1", "user1@email.com", None);
        let user_in_insert = user_in.clone().insert(&db.pool).await?;
        let user_out = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id=$1")
            .bind(user_in.clone().id)
            .fetch_one(&db.pool)
            .await?;
        assert_eq!(user_in, user_out);
        let user_out_delete = User::delete(&db.pool, user_out.id)
            .await.unwrap();
        assert!(user_out_delete.is_some());
        assert_eq!(Some(user_in_insert), user_out_delete);
        Ok(())
    }

    // NOTE this function can/should be generalized among any models
    #[actix::test]
    async fn inserts_many_ok() -> sqlx::Result<()> {
        let db = db().await.unwrap();
        let u1: User = user("user1", "user1@email.com", None)
            .insert(&db.pool).await?;
        let u2: User = user("user2", "user2@email.com", None)
            .insert(&db.pool).await?;
        let u3: User = user("user3", "user3@email.com", None)
            .insert(&db.pool).await?;
        let users: Vec<User> = User::get_all(&db.pool).await?;
        assert_eq!(users[0], u1);
        assert_eq!(users[1], u2);
        assert_eq!(users[2], u3);
        User::delete_all(&db.pool).await?;
        Ok(())

    }

}
