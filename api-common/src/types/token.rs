use sqlx::{self, Database, Type, TypeInfo};
use sqlx::prelude::*;
use uuid::Uuid;
use derive_more::{AsRef, AsMut, Display, From};

#[derive(sqlx::Type, From, AsRef, AsMut, Display)]
#[sqlx(transparent, type_name = "session_token")]
pub struct SessionToken(String);

#[derive(sqlx::Type, From, AsRef, AsMut, Display)]
#[sqlx(transparent, type_name = "access_token")]
pub struct AccessToken(String);


impl SessionToken {

    pub fn empty() -> Self {
        Self(String::new())
    }
}

impl AccessToken {

    pub fn empty() -> Self {
        Self(String::new())
    }
}

#[derive(sqlx::Type, From, AsRef, AsMut, Display)]
#[sqlx(transparent, type_name = "refresh_token")]
pub struct RefreshToken(String);

// impl<DB> Type<DB> for AccessToken{
/*
    fn type_info() -> <DB``::TypeInfo {
        <String as Type<DB>>::type_info()
    }

kkk} */
