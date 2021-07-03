use crate::{models::User, query::datetime::DateFilter, types::{
        Expiration, RefreshToken, Status,
        auth::{Provider, ProviderType},
        now, private, token::{Token, AccessToken}
    }};
use api_db::{Model, Id, Db};
use sqlx::{postgres::PgPool, FromRow, Postgres, types::chrono::{NaiveDateTime, Utc}};
use serde::{Serialize, Deserialize};


#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(rename_all = "snake_case")]
pub struct Account {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(default = "ProviderType::default")]
    pub provider_type: ProviderType,
    #[serde(default = "Provider::devisa_creds_provider_id")]
    pub provider_id: Provider,
    #[serde(default = "Id::nil")]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AccountQuery {
    pub provider_type: Option<ProviderType>,
    pub provider_id: Option<Provider>,
    pub access_token_expires_filters: Vec<DateFilter>,
    pub created_at_filters: Vec<DateFilter>,
    pub updated_at_filters: Vec<DateFilter>,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            id: Id::gen(),
            user_id: Id::nil(),
            provider_type: ProviderType::Credentials,
            provider_id: Provider::Devisa,
            provider_account_id: Id::nil(),
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

    #[inline]
    fn table() -> String { String::from("accounts") }
    #[inline]
    fn id_str() -> String { String::from("account_id") }

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
    #[inline]
    pub fn new_creds(user_id: Id, provider_account_id: Id, provider_id: Provider) -> Self {
        Self {
            id: Id::gen(),
            user_id,
            provider_account_id,
            provider_id,
            ..Default::default()
        }
    }
    #[inline]
    pub fn new_oauth(user_id: Id, provider_account_id: Id, provider_id: Provider) -> Self {
        Self {
            id: Id::gen(),
            user_id,
            provider_account_id,
            provider_id,
            ..Default::default()
        }
    }
    pub async fn get_by_provider_type(db: &PgPool, user_id: Id, p_type: ProviderType) -> sqlx::Result<Option<Self>> {
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
    pub async fn update_access_token(self, db: &PgPool, access_token: &AccessToken)
        -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>(
            "UPDATE    accounts
             SET       access_token = $1, updated_at = $2
             WHERE     id = $3
             RETURNING *")
            .bind(&access_token)
            .bind(now())
            .bind(&self.id)
            .fetch_one(db)
            .await?;
        Ok(res)
    }
    pub async fn update_refresh_token(self, db: &PgPool, refresh_token: &RefreshToken)
        -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>(
            "UPDATE    accounts
             SET       refresh_token = $1, updated_at = $2
             WHERE     id = $3
             RETURNING *")
            .bind(&refresh_token)
            .bind(now())
            .bind(&self.id)
            .fetch_one(db)
            .await?;
        Ok(res)
    }

    pub async fn get_user(self, db: &PgPool) -> sqlx::Result<User> {
        let res = User::get(db, self.user_id).await?
            .expect("(Infallible)");
        Ok(res)
    }

    pub fn new_devisa_creds_account(user_id: Id, creds_id: Id,) -> Account {
        tracing::info!("Creating new account..., user_id {}", &user_id);
        Account {
            id: Id::gen(),
            user_id,
            provider_type: ProviderType::Credentials,
            provider_id: Provider::Devisa,
            provider_account_id: creds_id,
            refresh_token: None,
            access_token: None,
            access_token_expires: None,
            created_at: now(),
            updated_at: now(),
        }
    }
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
        let acct = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM accounts WHERE provider_account_id = $1")
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



