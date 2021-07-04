use crate::{models::Model, types::Id};
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::{PgRow, PgPool},
    types::chrono::{NaiveDateTime, Utc}
};
use crate::now;

#[async_trait::async_trait]
pub trait Linked: Model + Default + Clone {

    type Left: Model + LinkedTo<Self::Right>;
    type Right: Model + LinkedTo<Self::Left>;

    fn link_id(self) -> Option<Id>;
    fn left_id(self) -> Id;
    fn right_id(self) -> Id;

    /// Default new function for join table consisting only of L, R ids + opt link_id
    /// Reimplement, or overwrite this function in the case of join table with other
    /// required fields
    fn new_basic(left_id: Id, right_id: Id, link_id: Option<Id>) -> Self;

    /// Default insert function for a join table consisting only of L, R ids + optional link_id
    ///     (and created, updated). Re-implement for join tables with extra required fields
    ///     ex. TopicCategory, etc.
    async fn insert_left_link(&self, db: &PgPool) -> sqlx::Result<Self> {
        let query_str = format!("
            INSERT INTO {link} ({left_id_str}, {right_id_str}, link_id )
            VALUES ($1, $2, $3)
            RETURNING *
            ",
            link = Self::table(),
            left_id_str = Self::Left::id_str(),
            right_id_str = Self::Right::id_str(),);
        let left = self.clone().left_id();
        let right = self.clone().right_id();
        let link = self.clone().link_id();
        let res = sqlx::query_as::<Postgres, Self>(&query_str)
            .bind(left)
            .bind(right)
            .bind(link)
            .fetch_one(db).await?;
        Ok(res)

    }

    async fn linked_to_left(db: &PgPool, left_id: Id) -> sqlx::Result<Vec<Self::Right>> {
        let query_str = format!("
            SELECT * FROM {right}
            INNER JOIN {link} ON {right}.id = {link}.{right_id_str}
            INNER JOIN {left} ON {left}.id = {link}.{left_id_str}
            WHERE {left}.id = $1
            ",
            left = Self::Left::table(),
            link = Self::table(),
            right = Self::Right::table(),
            left_id_str = Self::Left::id_str(),
            right_id_str = Self::Right::id_str(),);
        let res = sqlx::query_as::<Postgres, Self::Right>(&query_str)
            .bind(left_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    async fn linked_to_right(db: &PgPool, right_id: Id) -> sqlx::Result<Vec<Self::Left>> {
        let res = sqlx::query_as::<Postgres, Self::Left>(&format!("
            SELECT * FROM {left}
            INNER JOIN {link} ON {left}.id = {link}.{left_id_str}
            INNER JOIN {right} ON {right}.id = {link}.{right_id_str}
            WHERE {right}.id = $1
            ",
            left = Self::Left::table(),
            link = Self::table(),
            right = Self::Right::table(),
            left_id_str = Self::Left::id_str(),
            right_id_str = Self::Right::id_str(),
            ))
            .bind(right_id)
            .fetch_all(db).await?;
        Ok(res)
    }

    async fn linked_between(db: &PgPool, left_id: Id, right_id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("
                SELECT * FROM {link}
                WHERE {left_id} = $1
                 AND  {right_id} = $2
                ",
                link = Self::table(),
                left_id = Self::Left::id_str(),
                right_id = Self::Right::id_str()))
            .bind(left_id)
            .bind(right_id)
            .fetch_all(db).await?;
        Ok(res)
    }


}

#[async_trait::async_trait]
pub trait LinkedTo<L>
where
    L: Model + for<'r> FromRow<'r, PgRow>,
    Self: Model + for<'r> FromRow<'r, PgRow>
{

    type LinkModel: Model + for<'r> FromRow<'r, PgRow> + Linked;

    async fn get_entries_linked_to(db: &PgPool, other_id: Id) -> sqlx::Result<Vec<L>> {
        let res = sqlx::query_as::<Postgres, L>(&format!("
            SELECT * FROM {this}
            INNER JOIN {link} ON {this}.id = {link}.{this_id_str}
            INNER JOIN {other} ON {other}.id = {link}.{other_id_str}
            WHERE {other}.id = $1
            ",
            this = Self::table(),
            link = Self::LinkModel::table(),
            other = L::table(),
            this_id_str = Self::id_str(),
            other_id_str = L::id_str(),
            ))
            .bind(other_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    async fn get_links_to_entry(db: &PgPool, this_id: Id) -> sqlx::Result<Vec<L>> {
        let res = sqlx::query_as::<Postgres, L>(&format!("
            SELECT * FROM {other}
            INNER JOIN {link} ON {other}.id = {link}.{other_id_str}
            INNER JOIN {this} ON {this}.id = {link}.{this_id_str}
            WHERE {this}.id = $1
            ",
            this = Self::table(),
            link = Self::LinkModel::table(),
            other = L::table(),
            this_id_str = Self::id_str(),
            other_id_str = L::id_str(),
            ))
            .bind(this_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    async fn get_links_between(db: &PgPool, this_id: Id, other_id: Id) -> sqlx::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Self>(&format!("
                SELECT * FROM {link}
                WHERE {this_id_str} = $1
                 AND  {other_id_str} = $2
                ",
                link = Self::LinkModel::table(),
                this_id_str = Self::id_str(),
                other_id_str = L::id_str()))
            .bind(this_id)
            .bind(other_id)
            .fetch_all(db).await?;
        Ok(res)
    }
    /* async fn add_link(db: &PgPool, id: Id, other: L, link: Option<Link>) -> sqlx::Result<(Self::LinkModel, L)> {
        if let Some(link) = link {

        }
        match other.insert(db).await {
            Ok(l) => {


            }

        }

    } */
}
