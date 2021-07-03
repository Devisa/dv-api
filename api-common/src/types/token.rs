use std::convert::{TryFrom, TryInto};
use chrono::{Utc, NaiveDateTime, Duration};
use uuid::Uuid;
use sqlx::{PgPool, self};
use serde::{Serialize, Deserialize};
use api_db::{Id, Model};
use crate::auth::jwt::*;
use crate::models::Session;
use derive_more::{AsRef, AsMut, Display, From};
use super::{Expiration};


/// Until a more complex auth system is made,both tokens will just be the JWT
#[derive(sqlx::Type, PartialEq, Debug, Clone, Serialize, Deserialize, From, AsRef, AsMut, Display)]
#[sqlx(transparent, type_name = "session_token")]
pub struct SessionToken(String);

#[derive(sqlx::Type, PartialEq,Debug, Clone, Serialize, Deserialize, From, AsRef, AsMut, Display)]
#[sqlx(transparent, type_name = "access_token")]
pub struct AccessToken(String);


#[async_trait::async_trait]
pub trait Token {

    fn new(token: String) -> Self;

    fn nil() -> Self;

}


#[async_trait::async_trait]
impl Token for SessionToken {

    #[inline]
    fn new(token: String) -> Self {
        Self(token)
    }
    #[inline]
    fn nil() -> Self {
        Self(String::new())
    }
}

#[async_trait::async_trait]
impl Token for AccessToken {

    #[inline]
    fn new(token: String) -> Self {
        Self(token)
    }
    #[inline]
    fn nil() -> Self {
        Self(String::new())
    }
}

impl AccessToken {

    pub fn user_from_id(user_id: Id, session_id: Id, exp: Expiration) -> anyhow::Result<Self> {
        let exp = exp.hours_left() as u16;
        let issuer = String::from("dvsa-creds");
        let role = "user".to_string();
        let jwt = encode_token(user_id, session_id, issuer, role, exp)?;
        Ok(Self(jwt))
    }

    pub fn new_user(session: &Session) -> anyhow::Result<Self> {
        let exp = session.expires.hours_left() as u16;
        let issuer = String::from("dvsa-creds");
        let role = "user".to_string();
        let jwt = encode_token(session.clone().user_id, session.id.to_owned(), issuer, role, exp)?;
        Ok(Self(jwt))
    }

    pub fn decode(self) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode_token(self.get().as_str())
    }

    pub fn encoded_user(self) -> anyhow::Result<EncodedUser> {
        EncodedUser::try_from(self.decode()?)
    }

    pub fn is_expired(self) -> anyhow::Result<bool> {
        let claims = self.decode()?;
        let exp = NaiveDateTime::from_timestamp(claims.exp, 0);
        if exp - Utc::now().naive_utc() < Duration::zero() {
            Ok(true)
        } else {
            Ok(false)
        }

    }

    #[inline]
    pub fn get(self) -> String {
        self.0
    }
}

// TODO NOTE should have some user/account identifying info?
impl Default for SessionToken {
    #[inline]
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(sqlx::Type, PartialEq,Debug, Clone, Serialize, Deserialize, From, AsRef, AsMut, Display)]
#[sqlx(transparent, type_name = "refresh_token")]
pub struct RefreshToken(String);

impl Default for RefreshToken {
    #[inline]
    fn default() -> Self {
        RefreshToken(Uuid::new_v4().to_string())
    }
}

// impl<DB> Type<DB> for AccessToken{
/*
    fn type_info() -> <DB``::TypeInfo {
        <String as Type<DB>>::type_info()
    }

kkk} */
