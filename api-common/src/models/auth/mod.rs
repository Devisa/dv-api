use sqlx::{
    postgres::{Postgres, PgPool},
    types::chrono::NaiveDate,
};
use api_db::types::{Id, Model};
use crate::types::{now, AccessToken, SessionToken, RefreshToken};
use serde::{Serialize, Deserialize};

use super::{Account, Credentials, Profile, User, account::AccountProvider, credentials::CredentialsIn, user::UserIn};

/// For handling credentials-based signups in one transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialsSignupIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creds_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<Id>,
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>
}

impl Into<CredentialsIn> for CredentialsSignupIn {
    fn into(self) -> CredentialsIn {
        CredentialsIn {
            username: self.username,
            password: self.password,
        }
    }
}

impl Into<Profile> for CredentialsSignupIn {
    fn into(self) -> Profile {
        Profile {
            bio: self.bio,
            birthday: self.birthday,
            cover_image: self.cover_image,
            created_at: now(),
            updated_at: now(),
            ..Default::default()
        }
    }
}

impl Into<User> for CredentialsSignupIn {
    fn into(self) -> User {
        User {
            name: Some(self.name),
            image: self.image,
            email: Some(self.email),
            ..Default::default()
        }
    }
}

impl Into<Account> for CredentialsSignupIn {
    fn into(self) -> Account {
        Account {
            id: Id::nil(),
            user_id: self.user_id.unwrap_or(Id::nil()),
            ..Default::default()
        }
    }
}

impl CredentialsSignupIn {

    pub fn new(creds: CredentialsIn, user: UserIn ) -> Self {
        Self {
            user_id: None,
            email: user.email,
            image: user.image,
            name: user.name,
            cover_image: None,
            birthday: None,
            bio: None,
            password: creds.password,
            username: creds.username,
            city: None,
            country: None,
            profile_id: None,
            creds_id: None,
        }
    }

    /// A new user, not from another site or linked to an external ID,
    ///     signing up to the Devisa credentials user pool.
    ///     Must create: User, Credentials, (Devisa) Account, Profile
    ///     *In that order* -> VerificationRequest to confirm email
    ///     -> UserLevel for gamification element
    pub async fn signup_credentials(self, db: &PgPool) -> sqlx::Result<User> {
        let user = User::new(Some(self.name), Some(self.email), self.image)
            .insert(&db).await?;
        let creds = Credentials {
            id: Id::gen(),
            user_id: Id::from(user.clone().id),
            username: self.username,
            password: self.password
        }
            .hash()
            .insert(&db).await?;
        let profile: Profile = Profile {
            id: Id::gen(),
            user_id: user.clone().id,
            ..Default::default()
        }
            .insert(&db).await?;
        let acct: Account = Account::new_devisa_creds_account(
            Id::from(user.clone().id), creds.id,
        )
            .insert(&db).await?;
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

