use std::{
    collections::HashMap,
    sync::RwLock,
};
use uuid::Uuid;
use api_db::{Db, Id, Model};
use api_common::types::{Expiration, token::Token, AccessToken, SessionToken, now};
use actix_http::{StatusCode, header};
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError, dev::Payload, error::PayloadError, web};
use api_common::models::Session;
use chrono::{Duration, NaiveDateTime};
use futures_util::future::{ok, err, Ready};
use derive_more::{From, Display, Error};
use crate::error::TokenError;
use serde::{Serialize, Deserialize};

#[derive(Debug, From,  Clone,  Serialize, Deserialize)]
pub struct SessionIn {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id ,
    pub access_token: AccessToken,
    pub session_token: SessionToken,
    #[serde(default = "Expiration::two_days")]
    pub expires: Expiration,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

impl Default for SessionIn {
    fn default() -> Self {
        SessionIn {
            id: Id::gen(),
            user_id: Id::nil(),
            access_token: AccessToken::nil(),
            session_token: SessionToken::nil(),
            expires: Expiration::two_days(),
            created_at: now(),
            updated_at: now(),
        }
    }
}

impl Into<Session> for SessionIn {
    fn into(self) -> Session {
        Session {
            access_token: self.clone().access_token,
            user_id: self.clone().user_id,
            session_token: self.clone().session_token,
            expires: self.clone().expires,
            created_at: self.created_at,
            updated_at: self.updated_at,
            id: self.clone().id
        }
    }
}


impl FromRequest for SessionIn {
    type Error = TokenError;
    type Future = Ready<Result<Self, Self::Error>>;

    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let has_token = req.headers()
            .get(header::HeaderName::from_static("x-session-token"))
            .map(|val| val.to_str().ok())
            .ok_or(TokenError::MissingToken)
            .map_err(|e| {
                sentry::capture_error(&e);
                TokenError::MissingToken
            })
            .expect("Could not get token");
        if let Some(token) = has_token {
            ok(SessionIn::default())
        } else {
            err(TokenError::Internal)
        }
    }
}

#[derive(Debug, From)]
pub struct ApiSession {
    pub users: RwLock<HashMap<String, SessionInfo>>
}

impl Default for ApiSession {
    fn default() -> Self {
        Self {
            users: RwLock::new(HashMap::new())
        }
    }
}

impl ApiSession {

    pub fn set(input: &str, val: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn get(input: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn delete(input: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
#[derive(Debug, Clone, From)]
pub struct SessionInfo {
    pub user_id: Uuid,
    pub exp: NaiveDateTime,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub session_token: String,
    pub access_token: String,
}

impl Default for SessionInfo {
    fn default() -> Self {
        Self {
            exp: (chrono::Utc::now() + Duration::days(2)).naive_utc(),
            role: "user".to_string(),
            user_id: Uuid::nil(),
            created_at: chrono::Utc::now().naive_utc(),
            session_token: String::new(),
            access_token: String::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use actix_web::test::*;
    use super::*;

    #[actix_rt::test]
    async fn inserts_session_ok() -> () {}

}
