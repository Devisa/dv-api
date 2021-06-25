use uuid::Uuid;
use sqlx::{
    postgres::{Postgres, PgPool},
    types::chrono::NaiveDate,
};
use api_db::types::Model;
use crate::types::now;
use crate::auth::jwt::*;
use serde::{Serialize, Deserialize};

use super::{Account, Credentials, Profile, User, account::AccountProvider, credentials::CredentialsIn, user::UserIn};

/// For handling credentials-based signups in one transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialsSignupIn {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creds_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<Uuid>,
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
            id: Uuid::nil(),
            user_id: self.user_id.unwrap_or(Uuid::nil()),
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
        let creds: CredentialsIn = self.clone().into();
        let profile: Profile = self.clone().into();
        let user: User = self.clone().into();

        // 1. Create the user's base user model
        let user = user.insert(&db).await?; //Full struct with user id

        // 2. Use the created user to create credentials
        let creds = Credentials {
            id: Uuid::new_v4(),
            user_id: user.id,
            username: creds.username,
            password: creds.password
        };
        let creds = creds.hash().insert(&db).await?;  //Full struct with creds id
        // 3. Use the created credentials and user to create a Devisa acct
        let acct: Account = AccountProvider::devisa_creds_account(
            user.id,
            creds.id,
            None, // accesss token
            None, // refresh token
            None //Access token expires
        );
        acct.insert(&db).await?;

        // 4. Finally, create a new (likely empty) profile for the user
        let profile = Profile { user_id: user.id, ..profile };
        profile.insert(&db).await?;
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

pub async fn signup_credentials(db: &PgPool, user: User, creds: CredentialsIn) -> sqlx::Result<User> {

    let profile: Profile = Profile { user_id: user.id, ..Default::default() };
    let creds: Credentials = Credentials {
        user_id: user.id,
        username: creds.username,
        password: creds.password,
        id: Uuid::new_v4(),
    }.hash();
    // 1. Create the user's base user model

    // 2. Use the created user to create credentials
    let creds = Credentials {
        id: Uuid::new_v4(),
        user_id: user.id,
        username: creds.username,
        password: creds.password
    };
    let creds = creds.hash().insert(&db).await?;  //Full struct with creds id
    // 3. Use the created credentials and user to create a Devisa acct
    let acct: Account = AccountProvider::devisa_creds_account(
        user.id,
        creds.id,
        None, // accesss token
        None, // refresh token
        None //Access token expires
    );
    let user = user.insert(&db).await?; //Full struct with user id
    let acct = acct.insert(&db).await?;
    let creds = creds.insert(&db).await?;
    let profile = profile.insert(&db).await?;

    // 4. Finally, create a new (likely empty) profile for the user
    Ok(user)
}
