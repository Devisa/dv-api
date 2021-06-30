use api_db::Id;
use serde::{Serialize, Deserialize};
use crate::
    models::{
        Account,
    };
#[derive(sqlx::Type, Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
#[sqlx(type_name = "jwt")]
pub struct JWT(String);

impl Default for JWT {
    #[inline]
    fn default() -> Self {
        Self(String::new())
    }
}
#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "provider_type", rename_all = "lowercase")]
pub enum ProviderType {
    Credentials, OAuth,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "provider_id", rename_all = "lowercase")]
pub enum Provider {
    Devisa, Google, GitHub, GitLab, Facebook, LinkedIn, Twitter
}
impl Provider {
    #[inline]
    pub fn devisa_creds_provider_id() -> Self {
        Self::Devisa
    }
}
impl ProviderType {
    pub fn new_account(&self, user_id: Id, provider_account_id: Id) -> crate::models::Account {
        Account::default()
        /* match self {
            Self::Credentials => A,

        } */

    }
}
impl Default for Provider {
    #[inline]
    fn default() -> Self {
        Provider::Devisa
    }
}
impl Default for ProviderType {
    #[inline]
    fn default() -> Self {
        ProviderType::Credentials
    }
}
