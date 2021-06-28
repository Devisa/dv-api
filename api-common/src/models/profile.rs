use crate::types::{Gender, Role, now, GroupRole};
use api_db::{Db, Id};
use chrono::NaiveDate;
use url::Url;
use super::Model;
use serde::{Serialize, Deserialize};
use sqlx::{
    FromRow, Postgres, postgres::PgPool,
    types::chrono::{NaiveDateTime, Utc}
};


#[derive(Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default = "Role::default")]
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<NaiveDateTime>,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Profile {

    fn table() -> String { String::from("profiles") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
           INSERT INTO profiles (
            id,
            user_id,
            bio,
            birthday,
            gender,
            country,
            city,
            state,
            website,
            referral,
            company,
            occupation,
            twiter_url,
            facebook_url,
            education,
            linkedin_url,
            cover_image,
            postal_code,
            phone_number
            )
           VALUES (
           $1 ,$2 ,$3 ,$4 ,$5 ,$6 ,
           $7 ,$8 ,$9 ,$10 ,$11 ,$12 ,
           $13 ,$14, $15, $16, $17, $18, $19)
           RETURNING *
           ")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.bio)
            .bind(&self.birthday)
            .bind(&self.gender)
            .bind(&self.country)
            .bind(&self.city)
            .bind(&self.state)
            .bind(&self.website)
            .bind(&self.referral)
            .bind(&self.company)
            .bind(&self.occupation)
            .bind(&self.twitter_url)
            .bind(&self.facebook_url)
            .bind(&self.education)
            .bind(&self.linkedin_url)
            .bind(&self.cover_image)
            .bind(&self.postal_code)
            .bind(&self.phone_number)
            .fetch_one(db).await?;
        Ok(self)
    }
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            id: Id::gen(),
            user_id: Id::nil(),
            bio: None,
            cover_image: None,
            birthday: None,
            linkedin_url: None,
            facebook_url: None,
            education: None,
            twitter_url: None,
            city: None,
            role: Role::User,
            country: None,
            company: None,
            occupation: None,
            website: None,
            state: None,
            postal_code: None,
            phone_number: None,
            referral: None,
            gender: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),

        }
    }
}

impl Profile {

    /* pub async fn get_all(db: &PgPool) -> anyhow::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Profile>("SELECT * FROM profiles")
            .fetch_all(db).await?;
        Ok(res)
    }

    pub async fn get(db: &PgPool, id: Id) -> anyhow::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Profile>("SELECT * FROM profiles WHERE id = ,$1")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    } */

    pub async fn update<'a, T>(self, db: &PgPool, field: String, new_val: T)
        -> anyhow::Result<Self>
    where
        T: sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Send + 'a,
    {
        let res = sqlx::query_as::<Postgres, Self>("
            UPDATE profiles
            SET    $1 = $2
            WHERE  id = $3 RETURNING *
            ")
            .bind(field)
            .bind(new_val)
            .bind(self.id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn update_by_id<'a, T>(db: &PgPool, id: Id, field: String, new_val: T)
        -> anyhow::Result<Self>
    where
        T: sqlx::Encode<'a, Postgres> + sqlx::Type<Postgres> + Send + 'a,
    {
        let res = sqlx::query_as::<Postgres, Self>("
            UPDATE profiles
            SET $1 = $2
            WHERE id = $3 RETURNING *
            ")
            .bind(field)
            .bind(new_val)
            .bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn get_all_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Vec<Self>> {
        let res = sqlx::query_as::<Postgres, Profile>("SELECT * FROM profiles WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(db).await?;
        Ok(res)
    }

    /* pub async fn delete(db: &PgPool, id: Id) -> anyhow::Result<Option<Id>> {
        let res = sqlx::query_scalar("DELETE FROM profiles WHERE id = ,$1 RETURNING id")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(res)
    } */

}
