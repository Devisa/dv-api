use std::{str::FromStr, convert::Infallible};
use crate::Id;
use serde::{Serialize, Deserialize};
use crate::models::Account;

#[derive(sqlx::Type, Serialize, Deserialize, PartialEq, PartialOrd, Clone, Debug)]
#[sqlx(type_name = "jwt")]
pub struct JWT(String);

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "provider_type", rename_all = "lowercase")]
pub enum ProviderType {
    Credentials, OAuth,
}

#[derive(sqlx::Type, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[sqlx(type_name = "provider_id", rename_all = "lowercase")]
pub enum Provider {
    Devisa,
    Google,
    Github,
    Gitlab,
    Facebook,
    Linkedin,
    Twitter
}

impl From<&Provider> for ProviderType {
    #[inline(always)]
    fn from(provider: &Provider) -> Self {
        match provider {
            Provider::Devisa => ProviderType::Credentials,
            _ => ProviderType::OAuth,
        }
    }
}
impl FromStr for ProviderType {
    type Err = Infallible;
    #[inline(always)]
    fn from_str(provider_type: &str) -> Result<Self, Self::Err> {
        match provider_type {
            "oauth" => Ok(ProviderType::OAuth),
            _ => Ok(ProviderType::Credentials)
        }
    }
}

impl FromStr for Provider {
    type Err = Infallible;
    #[inline(always)]
    fn from_str(provider: &str) -> Result<Self, Self::Err> {
        match provider {
            "devisa" => Ok(Provider::Devisa),
            "google" => Ok(Provider::Google),
            "gitlab" => Ok(Provider::Gitlab),
            "linkedin" => Ok(Provider::Linkedin),
            "twitter" => Ok(Provider::Twitter),
            "github" => Ok(Provider::Github),
            "facebook" => Ok(Provider::Facebook),
            _ => Ok(Provider::Devisa)
        }
    }
}

impl Provider {
    #[inline]
    pub fn devisa_creds_provider_id() -> Self {
        Self::Devisa
    }
}
impl Default for JWT {
    #[inline]
    fn default() -> Self {
        Self(String::new())
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
