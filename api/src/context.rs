use protobuf::well_known_types::Api;
use redis::Client;
use api_db::Db;

#[derive(Debug, Clone)]
pub struct Context {
    pub db: Db,
    pub config: ApiConfig,
    pub redis: redis::Client,
}

impl Context {

    pub fn with_params(db: Db, config: ApiConfig, redis: redis::Client) -> Self {
        Self { db, config, redis }
    }

    pub async fn new() -> anyhow::Result<Self> {
        let config = ApiConfig::default()?;
        let db = Db::new(&config.db_url).await?;
        let redis = redis::Client::open(config.redis_url.as_str())?;
        Ok(Self { db, config, redis })
    }
}

#[derive(Debug, Clone)]
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

