use crate::types::{now, private};
use crate::models::Model;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::NaiveDateTime,
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub field_id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default = "Vec::new")]
    pub value: Vec<u8>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,

}

#[async_trait::async_trait]
impl Model for FieldTarget {

    fn table() -> String { String::from("field_targets") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>(
            "INSERT INTO field_target (field_id, name, description, value)
             vALUES ($1, $2)
             RETURNING *")
            .bind(self.field_id)
            .bind(&self.name)
            .bind(&self.description)
            .bind(self.value)
            .fetch_one(db).await?;
        Ok(res)
    }

}

impl FieldTarget {

    pub fn new(field_id: i32, name: String, description: Option<String>, value: Vec<u8>) -> Self {
        Self {
            updated_at: now(),
            created_at: now(),
            id: None,
            field_id, value, name, description
        }
    }


    pub async fn field(db: &PgPool, field_id: i32) -> sqlx::Result<Option<super::Field>> {
        let res = sqlx::query_as::<Postgres, super::Field>("
            SELECT * FROM fields WHERE id = $1")
            .bind(field_id)
            .fetch_optional(db).await?;
        Ok(res)
    }

}
