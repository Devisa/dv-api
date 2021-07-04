use crate::{Model, Id, models::ModelRoutes};
use actix_web::web::ServiceConfig;
use chrono::Duration;
use crate::{token::{AccessToken, SessionToken, Token}, Expiration, Status, now, private};
use sqlx::{postgres::PgPool, FromRow, Postgres, types::chrono::{NaiveDateTime, Utc}};
use serde::{Serialize, Deserialize};

#[derive(Debug,  FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
    #[serde(default = "Expiration::two_days")]
    pub expires: Expiration,
    #[serde(default = "SessionToken::default")]
    pub session_token: SessionToken,
    #[serde(default = "AccessToken::nil")]
    pub access_token: AccessToken,
    #[serde(default = "now")]
    pub created_at: NaiveDateTime,
    #[serde(default = "now")]
    pub updated_at: NaiveDateTime,
}

#[async_trait::async_trait]
impl Model for Session {
    fn table() -> String { "sessions".to_string() }

    async fn insert(
        self, db: &PgPool,) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO session (id, user_id, session_token, access_token, expires)
            VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(self.id)
            .bind(self.user_id)
            .bind(self.session_token)
            .bind(self.access_token)
            .bind(self.expires)
            .fetch_one(db)
            .await?;
        Ok(res)
    }

}
#[async_trait::async_trait]
impl ModelRoutes for Session {
    fn model_routes(cfg: &mut ServiceConfig) {
        cfg;
    }
}

impl Default for Session {
    fn default() -> Self {
        Session {
            id: Id::gen(),
            user_id: Id::nil(),
            session_token: SessionToken::nil(),
            access_token: AccessToken::nil(),
            expires: Expiration::two_days(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

impl Session {

    /* pub async fn get_all(db: &PgPool) -> anyhow::Result<Vec<Self>> {
        let sess = sqlx::query_as::<Postgres, Self>("SELECT * FROM sessions")
            .fetch_all(db).await?;
        Ok(sess)
    } */

    pub fn create(
        user_id: Id,
        exp: Expiration) -> anyhow::Result<Self>
    {
        let id = Id::gen();
        let access_token = AccessToken::user_from_id(user_id.clone(), id.clone(), exp.clone())?;
        Ok(Self {
            id,
            user_id,
            access_token,
            session_token: SessionToken::default(),
            expires: exp,
            ..Default::default()
        })
    }

    pub async fn remove(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            DELETE FROM sessions WHERE id = $1 returning *")
            .bind(&self.id)
            .fetch_one(db)
            .await?;
        Ok(res)

    }
    pub async fn is_expired(self, db: &PgPool, access_token: AccessToken) -> sqlx::Result<bool> {
        if self.expires.get() - Utc::now().naive_utc() < Duration::zero() {
            self.remove(db).await?;
            Ok(true)
        } else {
            Ok(false)
        }

    }

    pub fn set_session_token(self, token: String) -> Self {
        Self { session_token: SessionToken::default(), ..self }
    }

    pub fn set_access_token(mut self) -> anyhow::Result<()> {
        self.access_token = AccessToken::new_user(&self)?;

        Ok(())
    }

    pub fn get_access_token() -> Id {
        Id::gen()
    }
    pub fn get_session_token() -> Id {
        Id::gen()
    }

    pub async fn update_by_id(db: &PgPool, id: Id, r: Session)
        -> anyhow::Result<Self> {
        let res = sqlx::query_as::<Postgres, Self>("
            UPDATE     sessions
            SET        expires = $1
                       session_token = $2
                       access_token = $3
                       updated_at = $4
            WHERE      id = $5
            RETURNING  id
            ")
            .bind(&r.expires)
            .bind(&r.session_token)
            .bind(&r.access_token)
            .bind(now()).bind(id)
            .fetch_one(db).await?;
        Ok(res)
    }

    pub async fn get_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Option<Self>> {
        let sess = sqlx::query_as::<Postgres, Self>("SELECT * FROM sesions WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(sess)
    }

    //TODO do this more programmatically...
    pub async fn fetch_by_access_token(db: &PgPool, token: Id) -> anyhow::Result<Option<Self>> {
        let sess = sqlx::query_as::<Postgres, Self>("select * from sessions where access_token = $1")
            .bind(token)
            .fetch_optional(db).await?;
        Ok(sess)
    }
    pub async fn fetch_by_refresh_token(db: &PgPool, token: Id) -> anyhow::Result<Option<Self>> {
        let sess = sqlx::query_as::<Postgres, Self>("select * from sessions where refresh_token = $1")
            .bind(token)
            .fetch_optional(db).await?;
        Ok(sess)
    }

    pub async fn create_two_day_session(
        db: &PgPool,
        user_id: Id,
    ) -> anyhow::Result<Self> {
        let session_token = String::new();
        let access_token = String::new();
        let res = sqlx::query_as::<Postgres, Self>("
            INSERT INTO session (user_id, session_token, access_token, expires)
            VALUES ($1, $2, $3, $4) RETURNING *")
            .bind(user_id)
            .bind(session_token)
            .bind(access_token)
            .bind(Expiration::two_days())
            .fetch_one(db)
            .await?;
        Ok(res)
    }

    pub async fn delete_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Option<Self>> {
        let sess = sqlx::query_as::<Postgres, Self>("
            DELETE FROM sessions WHERE user_id = $1 returning id")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(sess)
    }

    /* pub async fn delete_by_id(db: &PgPool, id: Id) -> anyhow::Result<Option<Id>> {
        let sess = sqlx::query_scalar("DELETE FROM sessions WHERE id = $1 returning id")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(sess)
    } */

}
