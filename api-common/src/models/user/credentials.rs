use super::User;
use uuid::Uuid;
use pwhash::bcrypt::{BcryptSetup, BcryptVariant, self};
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Postgres, types::chrono::{NaiveDateTime, Utc}, postgres::PgPool};
use api_db::types::{Id, Model,};
use crate::types::{auth::{Provider, ProviderType}, now, AccessToken, SessionToken, RefreshToken};
use super::{Profile, Account,};

#[derive(PartialOrd,  Debug, FromRow, Clone, Serialize, Deserialize, PartialEq)]
#[sqlx(rename_all = "snake_case")]
pub struct Credentials {
    #[serde(default = "Id::gen")]
    pub id: Id,
    #[serde(default = "Id::nil")]
    pub user_id: Id,
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
    pub fn create(user_id: Id, username: String, password: String) -> Self {
        log::info!("CREATING NEW CREDENTIALS -- SIGNING UP NEW USER {}", &username);
        println!("CREATING NEW CREDENTIALS -- SIGNING UP NEW USER {}", &username);
        Self {
            id: Id::gen(),
            user_id,
            username,
            password: Self::hash_password(&password),
        }
    }

    /// To fetch existing credentials
    pub async fn fetch_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Option<Self>> {
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

    pub async fn get_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Option<Self>> {
        let creds = sqlx::query_as::<Postgres, Self>("SELECT * FROM credentials WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(creds)
    }

    pub async fn delete_by_user_id(db: &PgPool, user_id: Id) -> anyhow::Result<Option<Id>> {
        let creds = sqlx::query_scalar("DELETE FROM credentials WHERE user_id = $1 returning id")
            .bind(user_id)
            .fetch_optional(db).await?;
        Ok(creds)
    }

    pub async fn get_user(db: &PgPool, creds_id: Id) -> anyhow::Result<User> {
        let user = sqlx::query_as::<Postgres, User>("SELECT * FROM users WHERE id = $1")
            .bind(creds_id)
            .fetch_one(db).await?;
        Ok(user)
    }


    pub async fn verify(db: &PgPool, username: &str, password: &str)
        -> anyhow::Result<Credentials>
    {
        match sqlx::query_as::<Postgres, Credentials>("SELECT * FROM credentials WHERE username = $1")
            .bind(&username)
            .fetch_one(db).await
        {
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


/// For handling credentials-based signups in one transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialsSignup {
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: String,
}

impl Into<CredentialsIn> for CredentialsSignup {
    fn into(self) -> CredentialsIn {
        CredentialsIn {
            username: self.username,
            password: self.password,
        }
    }
}

impl Into<User> for CredentialsSignup {
    fn into(self) -> User {
        User {
            name: Some(self.name),
            email: Some(self.email),
            ..Default::default()
        }
    }
}

impl CredentialsSignup {

    pub fn new(username: &str, password: &str, name: &str, email: &str, ) -> Self {
        Self {
            email: email.into(),
            name: name.into(),
            password: password.into(),
            username: username.into(),
        }
    }

    /// A new user, not from another site or linked to an external ID,
    ///     signing up to the Devisa credentials user pool.
    ///     Must create: User, Credentials, (Devisa) Account, Profile
    ///     *In that order* -> VerificationRequest to confirm email
    ///     -> UserLevel for gamification element
    pub async fn signup_credentials(self, db: &PgPool) -> sqlx::Result<User> {
        let (user_id, cred_id) = ( Uuid::new_v4(), Uuid::new_v4() );
        let user: User = User {
            id: Id::new(user_id),
            name: Some(self.name),
            email: Some(self.email),
            ..Default::default()
        }
            .insert(db).await?;
        let creds = Credentials {
            id: Id::new(cred_id),
            user_id: Id::new(user_id),
            username: self.username,
            password: self.password
        }
            .hash()
            .insert(db).await?;
        let acct: Account = Account::new_devisa_creds_account(
            Id::new(user_id),
            Id::new(cred_id),
        )
            .insert(db).await?;
        let profile: Profile = Profile {
            id: Id::gen(),
            user_id: Id::new(user_id),
            ..Default::default()
        }
            .insert(db).await?;
        tracing::info!("NEW SIGNUP: The user has signed up: {:?}\n
                                    The credentials have signed up: {:?}\n
                                    The account has signed up: {:?}\n
                                    The profile hs signed up: {:?}\n",
                                &user, &creds, &acct, &profile);
        Ok(user)
    }

    /// A new user, signing up through Oauth (google, ex.) that is signing
    ///     up for their mandatory credentials. Upon signup, Next-Auth will
    ///     automatically handle: User, Account, Verif. , Account,
    ///     - simply need to create Credentials, Profile, and UserLevel
    pub async fn signup_oauth(self, db: &PgPool) -> sqlx::Result<()> {
        Ok(())
    }
}

