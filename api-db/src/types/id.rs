use std::convert::{TryFrom, TryInto};

use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::error::DdbError;
use derive_more::{Display,Deref, AsRef, Into};

        // PartialEq, Debug, Clone, Display, AsRef, AsMut)]

#[derive(sqlx::Type,Deref,Clone, Debug, Serialize, Deserialize, Display, PartialEq, PartialOrd)]
#[sqlx(transparent, type_name = "id")]
pub struct Id(String);

impl Default for Id {
    fn default() -> Id {
        Id::nil()
    }
}

impl Id {

    pub fn gen() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn is_nil(self) -> bool {
        self.0.parse::<Uuid>()
            .map(|u| u.is_nil())
            .map_err(|e| {
                tracing::error!(target: "db encoding", "Could not parse UUID {}", e);
                DdbError::ParseUuidError(e)
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
    type Error = DdbError;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        let id = Uuid::parse_str(&string)
            .map_err(|e| {
                tracing::info!("Eror decoding UUID: {}", e);
                DdbError::ParseUuidError(e)
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

