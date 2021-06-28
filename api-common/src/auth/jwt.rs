use std::fmt;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Validation, decode, Header, TokenData, DecodingKey, EncodingKey, Algorithm};
use anyhow::Result;
use chrono::{Duration, Local};
use std::convert::TryFrom;
use uuid::Uuid;
use api_db::Id;

#[derive(Debug, Clone, PartialEq)]
pub struct EncodedUser {
    pub user_id: Id,
    /// Session ID
    pub sid: Id,
    /// Site-wide role elevation
    pub role: String,
}

impl EncodedUser {

    pub fn new_user(uid: Id, sid: Id) -> Self {
        Self {
            role: Role::User.to_string(),
            sid, user_id: uid
        }
    }
}

#[derive(Clone)]
pub struct DecodedToken {
    pub jwt: Option<Claims>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// issuer
    pub iss: String,
    /// subject - Username
    pub sub: String,
    /// issued at
    pub iat: i64,
    /// expiry
    pub exp: i64,
    /// session id
    pub sid: String,
    /// user role
    pub role: String,
}

impl Claims {

    pub(crate) fn new(user_id: Id, session_id: Id, role: String, issuer: String, duration_hrs: u16) -> Self {
        let iat = Local::now();
        let exp = iat + Duration::hours(i64::from(duration_hrs));

        Claims {
            iss: issuer,
            sub: user_id.to_string(),
            sid: session_id.to_string().clone(),
            role: role.clone(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

#[derive(sqlx::Type, Debug, Clone)]
#[sqlx(transparent)]
pub struct JWT(String);

impl TryFrom<Claims> for EncodedUser {
    type Error = anyhow::Error;

    fn try_from(claims: Claims) -> Result<Self> {
        let Claims { sid, sub, role, .. } = claims;

        Ok(EncodedUser {
            sid: Id::try_from(sid)?,
            user_id: Id::try_from(sub)?,
            role,
        })
    }
}

impl JWT {

    pub fn create(
        user_id: Id,
        session_id: Id,
        issuer: String,
        role: String,
        duration_hrs: u16,
    ) -> anyhow::Result<Self> {
        let claims: Claims = Claims::new(user_id, session_id, issuer, role, duration_hrs);
        let token = encode(
            &Header::default(),
            &claims,
            &get_encoding_key()?,
        )?;
        Ok(Self(token))
    }
}


pub fn get_encoding_key() -> anyhow::Result<EncodingKey> {
    let key = if let Ok(env) = std::env::var("ENV") {
        match env.as_str() {
            "PROD" => std::env::var("JWT_SECRET")?,
            "DEV" => dotenv::var("JWT_SECRET")?,
            _ => panic!("ENV var not set correctly")
        }
    } else { dotenv::var("JWT_SECRET")? };
    return Ok(EncodingKey::from_secret(key.as_bytes()));
}

pub fn get_decoding_key() -> anyhow::Result<DecodingKey<'static>> {
    let key = if let Ok(env) = std::env::var("ENV") {
        match env.as_str() {
            "PROD" => std::env::var("JWT_SECRET")?,
            "DEV" => dotenv::var("JWT_SECRET")?,
            _ => panic!("ENV var not set correctly")
        }
    } else { dotenv::var("JWT_SECRET")? };
    return Ok(DecodingKey::from_base64_secret(key.as_str())?);
}

pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    let sec = if let Ok(sec) = std::env::var("JWT_SECRET") { sec }
    else { dotenv::var("JWT_SECRET").unwrap() };
    let sec = sec.as_str().as_bytes();
    let decoding_key = DecodingKey::from_secret(&sec);
    decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
}

pub fn decode_token_alt(token: &str) -> anyhow::Result<Claims> {
    println!("DECODING...: {}", token);
    let claims = decode::<Claims>(
        token,
        &get_decoding_key()?,
        &Validation::default()
    )
        .map(|data| data.claims)?;
    println!("DECODED: {}", token);
    Ok(claims)
}

pub fn encode_token(user_id: Id,
    session_id: Id,
    issuer: String,
    role: String,
    duration_hrs: u16,)  -> anyhow::Result<String>
{
    let claims: Claims = Claims::new(user_id, session_id, issuer, role, duration_hrs);
    let token = encode(
        &Header::default(),
        &claims,
        &get_encoding_key()?,
    )?;
    println!("ENCODED: {}", token);
    Ok(token)
}

#[derive(Clone, PartialEq)]
pub enum Role {
    User,
    Admin,
}

impl Role {

    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}


