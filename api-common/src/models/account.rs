use crate::{
    types::{Status, now, private}
};
use sqlx::{postgres::PgPool, FromRow, Postgres, types::chrono::{NaiveDateTime, Utc}};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub enum AccountProvider {
    /// id = "credentials"
    Devisa,
    /// id = "google"
    Google,
    /// id = "github"
    GitHub,
    /// id = "gitlab"
    GitLab,
    /// id = "facebook"
    Facebook,
    /// id = "linkedin"
    LinkedIn,
    /// id = "twitter" TODO
    Twitter
}

impl AccountProvider {

    pub fn devisa_account(user_id: Uuid, creds_id: Uuid) -> Account {
        let mut compound_id = String::from("devisa");
        compound_id.push_str(&creds_id.to_string());
        compound_id.push_str(&user_id.to_string());
        Account {
            id: Uuid::new_v4(),
            user_id,
            provider_type: "credentials".to_string(),
            provider_id: "credentials".to_string(),
            provider_account_id: creds_id.to_string(),
            refresh_token: None,
            access_token: None,
            access_token_expires: None,
            created_at: now(),
            updated_at: now(),
        }
    }
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Account {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub user_id: Uuid,
    pub provider_type: String,
    pub provider_id: String,
    pub provider_account_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub access_token_expires: Option<NaiveDateTime>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            id: Uuid::new_v4(),
            user_id: Uuid::nil(),
            provider_type: String::new(),
            provider_id: String::new(),
            provider_account_id: String::new(),
            refresh_token: None,
            access_token: None,
            access_token_expires: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[async_trait::async_trait]
impl super::Model for Account {

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
                access_token_expires)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
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
            .fetch_one(db).await?;
        Ok(acct)

    }

}

impl Account {

    /*
    pub async fn get_all(db: &PgPool) -> anyhow::Result<Vec<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts")
            .fetch_all(db).await?;
        Ok(acct)
    }

     pub async fn get_by_id(db: &PgPool, id: Uuid) -> anyhow::Result<Option<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE id = $1")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(acct)
    }

    pub async fn delete_by_id(db: &PgPool, id: Uuid) -> anyhow::Result<Option<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("DELETE FROM accounts WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(acct)
    }
    */

    pub async fn get_all_by_user_id(db: &PgPool, user_id: Uuid) -> anyhow::Result<Vec<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(acct)
    }

    pub async fn get_all_by_provider_id(db: &PgPool, user_id: Uuid) -> anyhow::Result<Vec<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("SELECT * FROM accounts WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(acct)
    }

    pub async fn insert_credentials(db: &PgPool, user_id: Uuid){}

    pub async fn delete_by_user_id(db: &PgPool, user_id: Uuid) -> anyhow::Result<Option<Uuid>> {
        let acct = sqlx::query_scalar("DELETE FROM accounts WHERE user_id = $1 returning id")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(acct)
    }
}

