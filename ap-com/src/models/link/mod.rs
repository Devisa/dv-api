use crate::{Id, Model};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::{PgRow, PgPool},
    types::chrono::{NaiveDateTime, Utc}
};
use crate::now;
#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Link {
    #[serde(default = "Id::gen")]
    pub id: Id,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Link {

    fn table() -> String { String::from("links") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Link> {
        let res = sqlx::query_as::<Postgres, Link>(
            "INSERT INTO links id, name, value, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.value)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .fetch_one(db).await?;
        Ok(res)

    }
}

impl Link {

    pub fn new(name: String, value: Option<String>) -> Self {
        Self { id: Id::gen(), name, value, created_at: now(), updated_at: now() }
    }

    pub async fn get_or_create_key_val(db: &PgPool, name: String, value: Option<String>) -> anyhow::Result<Self> {
        if let Some(value) = value {
            match sqlx::query_as::<Postgres, Link>("SELECT * FROM links WHERE name = $1, value = $2")
                .bind(&name)
                .bind(&value)
                .fetch_optional(db).await
            {
                Ok(Some(link)) => Ok(link),
                Ok(None) => {
                    let link = Link::new(name, Some(value)).insert(&db).await?;
                    Ok(link)
                },
                Err(e) => Err(anyhow::anyhow!("Error getting link"))
            }
        } else {
            match sqlx::query_as::<Postgres, Link>("SELECT * FROM links WHERE name = $1, value = null")
                .bind(&name)
                .fetch_optional(db).await
            {
                Ok(Some(link)) => Ok(link),
                Ok(None) => {
                    let link = Link::new(name, None).insert(&db).await?;
                    Ok(link)
                },
                Err(e) => Err(anyhow::anyhow!("Error getting link"))
            }
        }

    }

}
