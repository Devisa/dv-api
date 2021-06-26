use std::{collections::HashMap,};
use derive_more::{Display, From, Error};
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{Duration, NaiveDateTime};
use futures_util::lock::Mutex;
use protobuf::well_known_types::Api;
use redis::Client;
use api_db::Db;

#[derive(Debug,Clone )]
pub struct Context {
    pub db: Db,
    pub config: ApiConfig,
    pub redis: redis::Client,
    pub session: ApiSession,
}

#[derive(Debug, Clone, From)]
pub struct ApiSession {
    pub users: HashMap<String, SessionInfo>
}

impl Default for ApiSession {
    fn default() -> Self {
        Self {
            // users: Arc::new(HashMap::new())
            users: HashMap::new()
        }
    }
}

impl ApiSession {

    pub fn set(input: &str, val: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn get(input: &str) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn delete(input: &str) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, From)]
pub struct SessionInfo {
    pub user_id: Uuid,
    pub exp: NaiveDateTime,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub session_token: String,
    pub access_token: String,
}

impl Default for SessionInfo {
    fn default() -> Self {
        Self {
            exp: (chrono::Utc::now() + Duration::days(2)).naive_utc(),
            role: "user".to_string(),
            user_id: Uuid::nil(),
            created_at: chrono::Utc::now().naive_utc(),
            session_token: String::new(),
            access_token: String::new(),
        }
    }
}
impl Context {

    pub fn with_params(db: Db, config: ApiConfig, redis: redis::Client) -> Self {
        Self { db, config, redis, session: ApiSession::default() }
    }

    pub async fn new() -> anyhow::Result<Self> {
        let config = ApiConfig::default()?;
        let db = Db::new(&config.db_url).await?;
        let redis = redis::Client::open(config.redis_url.as_str())?;
        let session = ApiSession::default();
        Ok(Self { db, config, redis, session })
    }
}

#[derive(Debug,Clone )]
pub struct ApiConfig {
    pub port: u16,
    pub db_url: String,
    pub redis_url: String,
    pub prod: bool,
    pub host: String,
}

impl ApiConfig {

    pub fn with_params(port: u16, db_url: String, redis_url: String, prod: bool, host: String) -> Self {
        Self { port, db_url, redis_url, prod, host }
    }

    pub fn default() -> anyhow::Result<Self> {
        let db_url = if let Ok(env) = std::env::var("ENV") {
            match env.as_str() {
                "PROD" => { log::info!("ENV: PROD"); std::env::var("DATABASE_URL")? },
                "DEV" => { log::info!("ENV: DEV"); dotenv::var("DATABASE_URL")? } ,
                _ => panic!("Unknown ENV var")
            }
        } else { dotenv::var("DATABASE_URL")? };
        let port = if let Ok(port) = std::env::var("PORT") { port
        } else if let Ok(port) = dotenv::var("PORT") { port
        } else { "1888".to_string() };
        let port: u16 = port.parse()?;
        let redis_url: String = "redis://127.0.0.1".into();
        let host: String = "0.0.0.0".into();
        let prod = false;
        Ok(Self { port, db_url, redis_url, prod, host })
    }



}

