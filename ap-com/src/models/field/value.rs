use crate::{Status, now, private};
use crate::{Model, Id};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};

#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldValue {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub field_id: Id,
    #[serde(default = "Vec::new")]
    pub value: Vec<u8>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for FieldValue {

    fn table() -> String { String::from("field_values") }
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>(
            "INSERT INTO field_values (field_id, value)
             vALUES ($1, $2)
             RETURNING *")
            .bind(self.field_id)
            .bind(self.value)
            .fetch_one(db).await?;
        Ok(res)
    }

}

impl FieldValue {

    pub fn new(field_id: Id, value: Vec<u8>) -> Self {
        Self {
            updated_at: now(),
            created_at: now(),
            id: Id::gen(),
            field_id,
            value
        }
    }


    pub async fn field(db: &PgPool, field_id: Id) -> sqlx::Result<Option<super::Field>> {
        let res = sqlx::query_as::<Postgres, super::Field>("
            SELECT * FROM fields WHERE id = $1")
            .bind(field_id)
            .fetch_optional(db).await?;
        Ok(res)
    }

}
