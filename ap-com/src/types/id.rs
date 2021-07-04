use std::{ops, fmt, convert::{TryFrom, TryInto}};
use serde::{Serialize, Deserialize};
use sqlx::{Type, PgPool};
use uuid::Uuid;
use crate::error::DiLibError;

        // PartialEq, Debug, Clone, Display, AsRef, AsMut)]

#[derive(Type,Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
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
                DiLibError::ParseUuidError(e)
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
    type Error = DiLibError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&string)
            .map_err(|e| {
                tracing::info!("Eror decoding UUID: {}", e);
                DiLibError::ParseUuidError(e)
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
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for Id {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
impl std::convert::AsRef<String> for Id {
    fn as_ref(&self) -> &String {
        &self.0
    }
}
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
