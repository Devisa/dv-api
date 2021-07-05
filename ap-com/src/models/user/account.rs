use actix_web::web::{self, get, ServiceConfig};
use crate::{
    util::respond,
    models::{Credentials, User}, query::datetime::DateFilter, types::{
        Expiration, RefreshToken,
        auth::{Provider, ProviderType},
        now,  token::AccessToken
    }};
use crate::{Model, Id};
use sqlx::{postgres::PgPool, FromRow, Postgres, types::chrono::NaiveDateTime };
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

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
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
            created_at: now(),
            updated_at: now(),
        }
    }
}

#[async_trait::async_trait]
impl Model for Account {

    #[inline]
    fn table() -> String { String::from("accounts") }
    #[inline]
    fn id_str() -> String { String::from("account_id") }

    fn id(self) -> Id { self.id }

    /// Served as /user/account
    fn routes(cfg: &mut ServiceConfig) {
        cfg
            .route("/hi", get().to(|| respond::ok("GET /user/account/hi".to_string())));
    }

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
            .bind(self.id)
            .bind(self.user_id)
            .bind(self.provider_id)
            .bind(self.provider_type)
            .bind(self.provider_account_id)
            .bind(self.refresh_token)
            .bind(self.access_token)
            .bind(self.access_token_expires)
            .bind(self.created_at)
            .bind(self.updated_at)
            .fetch_one(db).await?;
        Ok(acct)
    }
}

impl Account {

    /// Create a new Credentials account struct
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
    /// Create a new Oauth account struct
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

    /// Get the user account with the designated provider type TODO more than one?
    pub async fn get_user_with_provider_type(
        db: &PgPool,
        user_id: Id,
        p_type: ProviderType
        ) -> sqlx::Result<Option<Self>>
    {
        let res = sqlx::query_as::<Postgres, Self>(
            "SELECT * FROM accounts WHERE
                 user_id = $1 AND
                 provider_type = $2
             RETURNING *
            ")
            .bind(user_id)
            .bind(p_type)
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

    /// Get the user associated with this account's user_id
    pub async fn get_user(self, db: &PgPool) -> sqlx::Result<User> {
        let res = User::get(db, self.user_id).await?
            .expect("(Infallible)");
        Ok(res)
    }

    /// Get the credentials associated with this account. If provider_type is not
    /// credentials, then return None
    pub async fn get_credentials(self, db: &PgPool) -> sqlx::Result<Option<Credentials>> {
        if self.provider_type == ProviderType::Credentials {
            let res = Credentials::get(db, self.provider_account_id)
                .await?
                .expect("Infallible");
            return Ok(Some(res))
        }
        Ok(None)
    }

    /// Create a new account where the provider type is local credentials
    #[inline(always)]
    pub fn new_devisa_credentials(user_id: Id, creds_id: Id,) -> Account {
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
    /// Get this account's access token
    #[inline(always)]
    pub fn access_token(self) -> Option<AccessToken> {
        self.access_token
    }
    /// Get this account's refresh token
    #[inline(always)]
    pub fn refresh_token(self) -> Option<RefreshToken> {
        self.refresh_token
    }
    /// Performed after creating new session after successful login from account
    #[inline(always)]
    pub fn set_access_token(mut self, at: AccessToken) {
        self.access_token = Some(at);
    }
    /// Set a new refresh token
    #[inline(always)]
    pub fn set_refresh_token(mut self, rt: RefreshToken) {
        self.refresh_token = Some(rt);
    }
    /// Create (but do not insert) a new account with provider and provider_account_id
    pub fn new_with_provider(
        user_id: Id,
        provider_account_id: Id,
        provider: Provider,
        access_token: Option<AccessToken>)
        -> Self
    {
        Self {
            id: Id::gen(),
            provider_type: ProviderType::from(&provider),
            provider_id: provider,
            user_id, provider_account_id,
            access_token,
            ..Default::default()
        }
    }
    /// Get account asociated with a provider account Id
    pub async fn get_by_provider_account_id(db: &PgPool, paid: Id
        ) -> sqlx::Result<Option<Self>>
    {
        let acct = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM accounts WHERE provider_account_id = $1")
            .bind(paid)
            .fetch_optional(db).await?;
        Ok(acct)
    }
    /// Delete account asociated with a provider account Id
    pub async fn delete_by_provider_account_id(db: &PgPool, paid: Id
        ) -> sqlx::Result<Option<Self>>
    {
        let acct = sqlx::query_as::<Postgres, Self>("
            DELETE FROM accounts WHERE provider_account_id = $1 RETURNING *")
            .bind(paid)
            .fetch_optional(db).await?;
        Ok(acct)
    }

    /// Get all accounts associated with a user
    pub async fn get_all_by_user_id(db: &PgPool, user_id: Id
        ) -> sqlx::Result<Vec<Self>>
    {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(acct)
    }

    /// Delete all accounts associated with a user
    pub async fn delete_all_by_user_id(db: &PgPool, user_id: Id
        ) -> sqlx::Result<Vec<Self>>
    {
        let acct = sqlx::query_as::<Postgres, Self>("
            DELETE FROM accounts WHERE user_id = $1 RETURNING *")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(acct)
    }

    /// Get all accounts in database with provider type, e.g. "credentials" or "oauth"
    pub async fn get_all_by_provider_type(db: &PgPool, ptype: ProviderType
        ) -> sqlx::Result<Vec<Self>>
    {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE provider_type = $1")
            .bind(ptype)
            .fetch_all(db).await?;
        Ok(acct)
    }

    /// Delete all accounts in database with provider type, e.g. "credentials" or "oauth"
    /// ```
    /// let devisa_accts = Account::delete_all_by_provider_type(db, "devisa").await?;
    /// ```
    pub async fn delete_all_by_provider_type(db: &PgPool, ptype: ProviderType
        ) -> sqlx::Result<Vec<Self>>
    {
        let acct = sqlx::query_as::<Postgres, Self>("
            DELETE FROM accounts WHERE provider_type = $1")
            .bind(ptype)
            .fetch_all(db).await?;
        Ok(acct)
    }
    /// Get all accounts in db with provider id, e.g. "devisa" or "gogle"
    pub async fn get_all_by_provider(db: &PgPool, provider: Provider
        ) -> sqlx::Result<Vec<Self>>
    {
        let acct = sqlx::query_as::<Postgres, Self>("
            SELECT * FROM accounts WHERE provider_id = $1")
            .bind(provider)
            .fetch_all(db).await?;
        Ok(acct)
    }

    /// Delete account associated with user_id
    pub async fn delete_all_by_provider(db: &PgPool, provider: Provider
        ) -> sqlx::Result<Vec<Self>>
    {
        let acct = sqlx::query_as::<Postgres, Self>(
            "DELETE FROM accounts WHERE provider_id = $1 returning *")
            .bind(provider)
            .fetch_all(db).await?;
        Ok(acct)
    }

    /// Check if this account is valid by checking if the associated user_id maps to a
    ///     valid user row, and if so, then checking that, if the account is a Credentials
    ///     account, the provider_account_id field maps to a valid credentials row
    pub async fn check_is_valid(self, db: &PgPool) -> sqlx::Result<bool> {
        let user_exists: Option<Id> = sqlx::query_scalar("SELECT id FROM users WHERE id = $1")
            .bind(&self.user_id)
            .fetch_optional(db)
            .await?;
        if let Some(_user_id) = user_exists {
            if self.provider_type == ProviderType::Credentials {
                if self.provider_id == Provider::Devisa {
                    let provider_account_id_exists: Option<Id> = sqlx::query_scalar("
                        SELECT id FROM credentials WHERE id = $1")
                        .bind(&self.provider_account_id)
                        .fetch_optional(db)
                        .await?;
                    return Ok(provider_account_id_exists.is_some());
                }
            }
        }
        return Ok(false);
    }
}

/* impl Responder for Account {
} */
