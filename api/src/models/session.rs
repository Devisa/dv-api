use api_db::{Db, Id, Model};
use api_common::types::now;
use actix_http::{StatusCode, header};
use actix_web::{FromRequest, HttpRequest, HttpResponse, ResponseError, dev::Payload, error::PayloadError, web};
use api_common::models::Session;
use api_common::types::Expiration;
use chrono::NaiveDateTime;
use futures_util::future::{ok, err, Ready};
use derive_more::{From, Display, Error};
use crate::error::SessionError;
use serde::{Serialize, Deserialize};

#[derive(Debug, From,  Clone,  Serialize, Deserialize)]
pub struct SessionIn {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id ,
    pub access_token: String,
    pub session_token: String,
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
            access_token: String::new(),
            session_token: String::new(),
            expires: Expiration::two_days(),
            created_at: now(),
            updated_at: now(),
        }
    }
}

impl Into<Session> for SessionIn {
    fn into(self) -> Session {
        Session {
            access_token: self.access_token.to_string(),
            user_id: self.clone().user_id,
            session_token: self.session_token.to_string(),
            expires: self.clone().expires,
            created_at: self.created_at,
            updated_at: self.updated_at,
            id: self.clone().id
        }
    }
}


impl FromRequest for SessionIn {
    type Error = SessionError;
    type Future = Ready<Result<Self, Self::Error>>;

    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let has_token = req.headers()
            .get(header::HeaderName::from_static("x-session-token"))
            .map(|val| val.to_str().ok())
            .ok_or(SessionError::MissingToken)
            .map_err(|e| {
                sentry::capture_error(&e);
                SessionError::MissingToken
            })
            .expect("Could not get token");
        if let Some(token) = has_token {
            ok(SessionIn::default())
        } else {
            err(SessionError::Internal)
        }
    }
}
