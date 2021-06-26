use tracing::info;
use crate::types::Model;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::prelude::*;
use sqlx::Executor;

#[derive(Clone, Debug)]
pub struct Db {
    pub pool: PgPool,
}

impl Db {

    pub async fn new(db_url: &str) -> anyhow::Result<Self> {
        let collector = tracing_subscriber::fmt::try_init()
            .expect("Could not init tracing subscriber");
        let pool = PgPoolOptions::new()
            .connect(db_url).await?;
        let psize = pool.size();
        info!(psize, "DB Pool successfully initialized");
        Ok( Self { pool } )
    }

    pub async fn clear(self, table: &str) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM $1 RETURNING id")
            .bind(&table)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get<T>(self, id: i32) -> sqlx::Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, PgRow> + Model + Unpin + Send
    {
        let res = sqlx::query_as::<sqlx::Postgres, T>("SELECT * FROM $1 WHERE id = $2")
            .bind(T::table())
            .bind(id)
            .fetch_optional(&self.pool).await?;
        Ok(res)
    }

    pub async fn get_all<T>(self) -> sqlx::Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, PgRow> + Model + Unpin + Send
    {

        let res = sqlx::query_as::<sqlx::Postgres, T>("SELECT * FROM $1")
            .bind(T::table())
            .fetch_all(&self.pool).await?;
        Ok(res)
    }

    pub async fn get_all_by_user<T>(self, user_id: i32) -> sqlx::Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, PgRow> + Model + Unpin + Send
    {

        let res = sqlx::query_as::<sqlx::Postgres, T>("SELECT * FROM $1 WHERE user_id = $2")
            .bind(T::table())
            .bind(user_id)
            .fetch_all(&self.pool).await?;
        Ok(res)
    }

    pub async fn get_all_with_field_val<T, S>(self, field: &str, val: S) -> sqlx::Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, PgRow> + Model + Unpin + Send,
        S: std::string::ToString,
    {

        let res = sqlx::query_as::<sqlx::Postgres, T>("SELECT * FROM $1 WHERE $2 = $3")
            .bind(T::table())
            .bind(field)
            .bind(val.to_string())
            .fetch_all(&self.pool).await?;
        Ok(res)
    }

    pub async fn delete<T>(self, id: i32) -> sqlx::Result<Option<T>>
    where
        T: for<'r> sqlx::FromRow<'r, PgRow> + Model + Unpin + Send
    {
        let res = sqlx::query_as::<sqlx::Postgres, T>("DELETE FROM $1 WHERE id = $2 RETURNING *")
            .bind(T::table())
            .bind(id)
            .fetch_optional(&self.pool).await?;
        Ok(res)
    }

    pub async fn delete_all<T>(self) -> sqlx::Result<Vec<T>>
    where
        T: for<'r> sqlx::FromRow<'r, PgRow> + Model + Unpin + Send
    {
        let res = sqlx::query_as::<sqlx::Postgres, T>("DELETE FROM $1 RETURNING *")
            .bind(T::table())
            .fetch_all(&self.pool).await?;
        Ok(res)
    }
}
