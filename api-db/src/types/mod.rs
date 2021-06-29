pub mod id;

pub use id::Id;
use sqlx::{
    prelude::*, FromRow, Postgres, PgPool,
    postgres::PgRow,
    types::chrono::NaiveDateTime,
};

#[async_trait::async_trait]
pub trait Model
where
    Self: Sized + for<'r> FromRow<'r, PgRow> + Unpin + Send {

    /// Return corresponding table string
    fn table() -> String;

    /// Returns what this field's id would be called as a foreign key on another table
    ///     e.g. user_id for users
    ///     Overwrite for tables where this doesn't apply -- ex. categories vs. category_id
    fn id_str() -> String {
        let mut out = Self::table();
        out.pop();
        out.push_str("_id");
        return out;
    }


    /// Insert the model into the database
    async fn insert(self, db: &PgPool) -> sqlx::Result<Self>;

    async fn get(db: &PgPool, id: Id) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("SELECT * FROM {} WHERE id = $1", Self::table()))
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    async fn get_all(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("SELECT * FROM {}", Self::table()))
            .fetch_all(db).await?;
        Ok(res)
    }

    async fn delete(db: &PgPool, id: Id) -> sqlx::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} WHERE id = $1 RETURNING *", Self::table()))
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    async fn delete_all(db: &PgPool) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} RETURNING *", Self::table()))
            .fetch_all(db).await?;
        Ok(res)
    }

    async fn get_by_id(self, db: &PgPool, kind: &str, id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("SELECT * FROM {} WHERE {}_id = $1", Self::table(), kind))
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    async fn delete_by_id(self, db: &PgPool, kind: &str, id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} WHERE {}_id = $1", Self::table(), kind))
            .bind(id)
            .fetch_all(db).await?;
        Ok(res)
    }

    async fn get_after(self, db: &PgPool, datetime: NaiveDateTime) -> sqlx::Result<Vec<Self>> {
        /* let res = sqlx::query_as::<Postgres, Self>(&format!("DELETE FROM {} WHERE {}_id = $1", Self::table(), kind))
            .bind(id)
            .fetch_all(db).await?; */
        Ok(Vec::new())
    }


}

pub async fn get<'r, F: 'r + FromRow<'r, PgRow>>() {

}
/*
impl<'r> sqlx::FromRow<'r, PgRow> for dyn Model {
} */

/* pub trait ManyToOne<T: Model>
where
    Self: Sized + for<'r> FromRow<'r, PgRow> + Unpin + Send ,
    T: Sized + for<'r> FromRow<'r, PgRow> + Unpin + Send
{

} */
