use uuid::Uuid;
use api_db::types::Model;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::{PgRow, PgPool},
    types::chrono::{NaiveDateTime, Utc}
};
use crate::types::now;

#[async_trait::async_trait]
pub trait Linked: Model + Default + Clone {

    type Left: Model + LinkedTo<Self::Right>;
    type Right: Model + LinkedTo<Self::Left>;

    fn link_id(self) -> Option<Uuid>;
    fn left_id(self) -> Uuid;
    fn right_id(self) -> Uuid;

    /// Default new function for join table consisting only of L, R ids + opt link_id
    /// Reimplement, or overwrite this function in the case of join table with other
    /// required fields
    fn new_basic(left_id: Uuid, right_id: Uuid, link_id: Option<Uuid>) -> Self;

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

    async fn linked_to_left(db: &PgPool, left_id: Uuid) -> sqlx::Result<Vec<Self::Right>> {
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
    async fn linked_to_right(db: &PgPool, right_id: Uuid) -> sqlx::Result<Vec<Self::Left>> {
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

    async fn linked_between(db: &PgPool, left_id: Uuid, right_id: Uuid) -> sqlx::Result<Vec<Self>> {
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

    async fn get_entries_linked_to(db: &PgPool, other_id: Uuid) -> sqlx::Result<Vec<L>> {
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
    async fn get_links_to_entry(db: &PgPool, this_id: Uuid) -> sqlx::Result<Vec<L>> {
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
    async fn get_links_between(db: &PgPool, this_id: Uuid, other_id: Uuid) -> sqlx::Result<Vec<Self>> {
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
    /* async fn add_link(db: &PgPool, id: Uuid, other: L, link: Option<Link>) -> sqlx::Result<(Self::LinkModel, L)> {
        if let Some(link) = link {

        }
        match other.insert(db).await {
            Ok(l) => {


            }

        }

    } */
}


#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Link {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
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
        Self { id: Uuid::new_v4(), name, value, created_at: now(), updated_at: now() }
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
