use super::User;
use pwhash::bcrypt::{BcryptSetup, BcryptVariant, self};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Postgres, types::chrono::{NaiveDateTime, Utc}, postgres::PgPool};
use uuid::Uuid;

#[derive(PartialOrd,  Eq, Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
pub struct Credentials {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "Uuid::nil", skip_serializing_if="Uuid::is_nil")]
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

#[derive(FromRow, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CredentialsIn {
    pub username: String,
    pub password: String,
}

#[async_trait::async_trait]
impl super::Model for Credentials {

    fn table() -> String { String::from("credentials") }

    async fn insert(self, db: &PgPool) -> sqlx::Result<Self> {
        let res = sqlx::query_scalar("INSERT INTO credentials
        (id, user_id, username, password) VALUES ($1, $2, $3, $4)
            RETURNING id")
            .bind(&self.id)
            .bind(&self.user_id)
            .bind(&self.username)
            .bind(&self.password)
            .fetch_one(db).await?;
        Ok(self)
    }

}

impl CredentialsIn {

    pub fn hash(self) -> Self {
        CredentialsIn { password: Self::hash_password(&self.password), ..self }
    }

    pub fn hash_password(password: &str) -> String {
        bcrypt::hash_with(BcryptSetup {
            variant: Some(BcryptVariant::V2y), ..Default::default()
        }, password)
            .expect("Could not hash password")
    }

    /// Inserts the credentials input into the Database into the Credentials
    ///     table. If no user_id is known in the CredentialsIn object,
    ///     it is fetched from the corresponding user using a username
    ///     lookup.
    pub async fn login(self, db: &PgPool) -> anyhow::Result<Credentials> {
        if let Some(creds) = Credentials::fetch_by_username(db, self.username).await? {
            if Self::hash_password(&self.password) == creds.password {
                return Ok(creds);
            } else {
                return Err(anyhow::anyhow!("Password did not match"));
            }
        }
        return Err(anyhow::anyhow!("No user with that username"))
    }
}


impl Credentials {

    /// To create _new_ credentials -- used for signup
    pub fn create(user_id: Uuid, username: String, password: String) -> Self {
        log::info!("CREATING NEW CREDENTIALS -- SIGNING UP NEW USER {}", &username);
        println!("CREATING NEW CREDENTIALS -- SIGNING UP NEW USER {}", &username);
        Self {
            id: uuid::Uuid::new_v4(),
            user_id,
            username,
            password: Self::hash_password(&password),
        }
    }

    /// To fetch existing credentials
    pub async fn fetch_by_user_id(db: &PgPool, user_id: Uuid) -> anyhow::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Credentials>("
            SELECT * FROM credentials WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(res)
    }

    /// To fetch existing credentials
    pub async fn fetch_by_username(db: &PgPool, username: String) -> anyhow::Result<Option<Self>> {
        let res = sqlx::query_as::<Postgres, Credentials>("
            SELECT * FROM credentials WHERE username = $1")
            .bind(username)
            .fetch_optional(db).await?;
        Ok(res)
    }

    pub fn hash(self) -> Self {
        Credentials { password: Self::hash_password(&self.password), ..self }
    }

    pub fn hash_password(password: &str) -> String {
        bcrypt::hash_with(BcryptSetup {
            variant: Some(BcryptVariant::V2y), ..Default::default()
        }, password)
            .expect("Could not hash password")
    }

    /* pub async fn delete_by_id(db: &PgPool, id: Uuid) -> anyhow::Result<Option<Self>> {
        let acct = sqlx::query_as::<Postgres, Self>("DELETE FROM credentials WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_optional(db).await?;
        Ok(acct)
    }

    pub async fn get_all(db: &PgPool) -> anyhow::Result<Vec<Self>> {
        let creds = sqlx::query_as::<Postgres, Self>("SELECT * FROM credentials")
            .fetch_all(db).await?;
        Ok(creds)
    }
    pub async fn get_by_user_id(db: &PgPool, user_id: Uuid) -> anyhow::Result<Option<Self>> {
        let creds = sqlx::query_as::<Postgres, Self>("SELECT * FROM credentials WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(creds)
    }
 */
    pub async fn delete_by_user_id(db: &PgPool, user_id: Uuid) -> anyhow::Result<Option<Uuid>> {
        let creds = sqlx::query_scalar("DELETE FROM credentials WHERE user_id = $1 returning id")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(creds)
    }

    pub async fn get_user(db: &PgPool, creds_id: Uuid) -> anyhow::Result<User> {
        let user = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id = $1")
            .bind(creds_id)
            .fetch_one(db).await?;
        Ok(user)
    }


    pub async fn verify(db: &PgPool, username: &str, password: &str) -> anyhow::Result<Credentials> {
        match sqlx::query_as::<Postgres, Credentials>("SELECT * FROM credentials WHERE username = $1")
            .bind(&username)
            .fetch_one(db).await {
                Ok(user) => {
                    if bcrypt::verify(password, &user.password) {
                        Ok(user)
                    } else {
                        Err(anyhow::anyhow!("The username and/or password were incorrect"))
                    }
                }
                Err(e) => {
                    Err(anyhow::anyhow!("User with that username does not exist {}", e))
                }

            }
    }

}
