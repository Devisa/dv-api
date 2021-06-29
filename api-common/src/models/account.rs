use crate::types::{token::{Token, AccessToken}, Expiration, RefreshToken, Status, now, private};
use api_db::{Model, Id, Db};
use sqlx::{postgres::PgPool, FromRow, Postgres, types::chrono::{NaiveDateTime, Utc}};
use serde::{Serialize, Deserialize};
use derive_more::{FromStr, Display};
pub use crate::types::{ProviderType, Provider};


#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    pub provider_type: String,
    #[serde(default = "Provider::devisa_creds_provider_id")]
    pub provider_id: Provider,
    #[serde(default = "Id::gen")]
    pub provider_account_id: Id,
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_token: Option<RefreshToken>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<AccessToken>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token_expires: Option<Expiration>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            id: Id::gen(),
            user_id: Id::nil(),
            provider_type: "credentials".to_string(),
            provider_id: Provider::Devisa,
            provider_account_id: Id::gen(),
            refresh_token: None,
            access_token: None,
            access_token_expires: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[async_trait::async_trait]
impl Model for Account {

    fn table() -> String { String::from("accounts") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let acct = sqlx::query_as::<Postgres, Self>("
            INSERT INTO accounts (
                id,
                user_id,
                provider_id,
                provider_type,
                provider_account_id,
                refresh_token,
                access_token,
                access_token_expires,
                created_at,
                updated_at
                )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            ")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.provider_id)
            .bind(&self.provider_type)
            .bind(&self.provider_account_id)
            .bind(&self.refresh_token)
            .bind(&self.access_token)
            .bind(&self.access_token_expires)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(acct)

    }

}

impl Account {

    pub fn new_google_account(user_id: Id,creds_id: Id,) -> Account {
        Self::default()
    }

    pub async fn get_credentials_account(db: &PgPool, user_id: Id) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(
            "SELECT * FROM accounts WHERE
                 user_id = $1 AND
                 provider_type = 'credentials'
             RETURNING *
            ")
            .bind(user_id)
            .fetch_optional(db)
            .await?;
        Ok(res)
    }

    pub async fn update_creds_access_token(
        db: &PgPool,
        user_id: &Id,
        access_token: &AccessToken
    ) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(
            "UPDATE accounts
             SET
                access_token = $1
                updated_at = $2
             WHERE
                 user_id = $3 AND
                 provider_type = 'credentials'
             RETURNING *
            ")
            .bind(&access_token)
            .bind(now())
            .bind(&user_id)
            .fetch_optional(db)
            .await?;
        Ok(res)
    }

    pub fn new_devisa_creds_account(user_id: Id, creds_id: Id,) -> Account {
        tracing::info!("Creating new account..., user_id {}", &user_id);
        Account {
            id: Id::gen(),
            user_id,
            provider_type: "credentials".to_string(),
            provider_id: Provider::Devisa,
            provider_account_id: creds_id,
            refresh_token: None,
            access_token: None,
            access_token_expires: None,
            created_at: now(),
            updated_at: now(),
        }
    }
    /*
    pub async fn get_all(db: &PgPool) -> anyhow::Result<Vec<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts")
            .fetch_all(db).await?;
        Ok(acct)
    }

     pub async fn get_by_id(db: &PgPool, id: Id) -> anyhow::Result<Option<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE id = $1")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(acct)
    }

    pub async fn delete_by_id(db: &PgPool, id: Id) -> anyhow::Result<Option<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("DELETE FROM accounts WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(acct)
    }
    */

    pub fn access_token(self) -> Option<AccessToken> {
        self.access_token
    }

    /// Performed after creating new session after successful login from account
    pub fn set_access_token(mut self, at: AccessToken) {
        self.access_token = Some(at);
    }

    pub fn new_google() {
    }

    pub fn new_github() {}

    pub async fn grant_access_token(self, token: String) -> anyhow::Result<Self> {
        Ok(self)
    }

    pub async fn grant_refresh_token(self, token: String) -> anyhow::Result<Self> {
        Ok(self)
    }

    pub fn new_dvsa_creds(user_id: Id, access_token: String, access_token_expires: Option<NaiveDateTime>) {

    }

    pub async fn get_by_provider_account_id(db: &PgPool, paid: Id) -> anyhow::Result<Option<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE provider_account_id = $1")
            .bind(paid)
            .fetch_optional(db).await?;
        Ok(acct)
    }

    pub async fn get_all_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Vec<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(acct)
    }

    pub async fn get_all_by_provider_type(db: &PgPool, ptype: &str) -> anyhow::Result<Vec<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE provider_type = $1")
            .bind(ptype)
            .fetch_all(db).await?;
        Ok(acct)
    }
    pub async fn get_all_by_provider_id(db: &PgPool, pid: Id) -> anyhow::Result<Vec<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE provider_id = $1")
            .bind(pid)
            .fetch_all(db).await?;
        Ok(acct)
    }

    pub async fn insert_credentials(db: &PgPool, user_id: Id){}

    pub async fn delete_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Option<Id>> {
        let acct = sqlx::query_scalar("DELETE FROM accounts WHERE user_id = $1 returning id")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(acct)
    }
}



