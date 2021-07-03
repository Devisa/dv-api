use std::{ops, fmt, convert::{TryFrom, TryInto}};
use futures_util::future::{ok, err, Ready};
use crate::{error::ParseError, ApiError};
use actix_http::Payload;
use futures_util::Future;
use serde::{Serialize, Deserialize};
use sqlx::{Type, PgPool};
use uuid::Uuid;
use actix_web::{FromRequest, HttpRequest, web::{self, Path}};

#[derive(Type,Clone, Debug, Serialize, Deserialize,  PartialEq, PartialOrd)]
#[sqlx(transparent, type_name = "id")]
pub struct Id(String);

impl Default for Id {
    fn default() -> Id {
        Id::nil()
    }
}

impl Id {

    pub fn new(uid: uuid::Uuid) -> Self {
        Self(uid.to_string())
    }

    pub fn gen() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn is_nil(self) -> bool {
        self.0.parse::<Uuid>()
            .map(|u| u.is_nil())
            .map_err(|e| {
                tracing::error!(target: "db encoding", "Could not parse UUID {}", e);
                ApiError::ParseError(ParseError::Uuid(e))
            })
            .unwrap_or(true)
    }

    pub fn nil() -> Self {
        Self(Uuid::nil().to_string())
    }
    pub fn get(self) -> String {
        self.0
    }

    pub async fn verify_existence(self, db: &PgPool, table: &str) -> anyhow::Result<bool> {
        let res = format!("SELECT * FROM {} WHERE id = $1", table);
        let out = sqlx::query(res.as_str())
            .bind(self.get())
            .fetch_optional(db).await?;
        if let Some(_row) = out {
            Ok(true)
        } else {
            Ok(false)
        }
    }

}

impl TryFrom<String> for Id {
    type Error = ApiError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&string)
            .map_err(|e| {
                tracing::info!("Eror decoding UUID: {}", e);
                ApiError::ParseError(ParseError::Uuid(e))
            })?
            .to_string();
        Ok(Id(id))
    }
}

impl From<Uuid> for Id {
    fn from(guid: Uuid) -> Self {
        Self(guid.to_string())
    }
}

impl From<&Id> for Id {
    fn from(id: &Id) -> Self {
        Self(id.clone().get())
    }
}
impl Into<Uuid> for Id {
    fn into(self) -> Uuid {
        Uuid::parse_str(self.get().as_str())
            .expect("Could not convert ID str to UUID")
    }
}
impl ops::Deref for Id {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl ops::DerefMut for Id {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl FromRequest for Id {
    type Config = ();
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req = req.head().headers.get("dvsa-user-id");
        if let Some(user_id) = req {
            return ok(Id(user_id.to_str().unwrap().to_string()));
        } else {
            return err(ApiError::MissingParam { param: "dvsa-user-id".to_string() });
        }
    }
}
