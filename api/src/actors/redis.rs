use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::{aio::Connection, FromRedisValue, AsyncCommands};
// use redis::{Client, aio::MultiplexedConnection};
use std::time::Duration;
use actix::prelude::*;
use log::{info, warn};
use tokio::process::Command;

#[derive(Clone, Debug)]
pub struct RedisPool {

}

#[derive(Clone, Debug)]
pub struct RedisActor {
    // conn: MultiplexedConnection,
    client: redis::Client,
}

impl RedisActor {

    pub fn creds() -> anyhow::Result<(String, String)> {
        Ok((dotenv::var("REDIS_HOST")?,
        dotenv::var("REDIS_PASS")?))
    }

    pub async fn new() -> anyhow::Result<Self> {
        if let Ok(( host, pass )) = Self::creds() {
            let client = redis::Client::open(host.as_str())?;
            log::info!("SPAWNED NEW REDIS ACTOR.");
            Ok(Self { client })
        } else {
            log::info!("NO ENV VARS FOR REDIS -- STARTING FROM DEFAULT");
            Self::spawn().await?;
            let (host, _pass)  = ("redis://127.0.0.1:6379", "");
            let client = redis::Client::open(host)?;
            Ok(Self { client })
        }
    }

    pub async fn spawn() -> anyhow::Result<()> {
        Command::new("redis-server").output().await?;
        Ok(())
    }

    pub async fn conn(&self) -> anyhow::Result<redis::aio::Connection> {
        log::info!("NEW REDIS ACTOR CONNECTION");
        let conn = self.client.get_async_connection().await?;
        Ok(conn)
    }

    pub async fn conn_multi(&self) -> anyhow::Result<redis::aio::MultiplexedConnection> {
        log::info!("NEW REDIS ACTOR MULTIPLEXED CONNECTION");
        let conn = self.client
            .get_multiplexed_tokio_connection().await?;
        Ok(conn)
    }

    pub async fn set(&self, key: &str, val: &str) -> anyhow::Result<()> {
        let mut conn = self.clone().conn().await?;
        let _res = conn.set(key, val).await?;
        Ok(())
    }
    pub async fn get(&self, key: &str) -> anyhow::Result<String> {
        let mut conn = self.conn().await?;
        let res: String = conn.get(key).await?;
        Ok(res)
    }
    pub async fn del(&self, key: &str) -> anyhow::Result<()> {
        let mut conn = self.conn().await?;
        let _res = conn.del(key).await?;
        Ok(())
    }
    pub async fn keys(&self, patt: &str) -> anyhow::Result<Vec<String>> {
        let r = self.conn().await?
            .keys(patt).await?;
        Ok(r)
    }
    pub async fn conn_pubsub(&self) -> anyhow::Result<redis::aio::PubSub> {
        log::info!("CREATED NEW REDIS ACTOR PUBSU");
        let conn = self.client.get_tokio_connection().await?
            .into_pubsub();
        Ok(conn)
    }

    pub async fn conn_monitor(&self) -> anyhow::Result<redis::aio::Monitor> {
        log::info!("CREATED NEW REDIS ACTOR MONITOR CONNECTION");
        let conn = self.client.get_tokio_connection().await?
            .into_monitor();
        Ok(conn)
    }
}



// impl RedisActor {
//     pub async fn new(redis_url: &'static str) -> Self {
//         let client = Client::open(redis_url).unwrap();
//         let conn = client.get_connection().expect("COULDNT GET CONN");
//         info!(target: "redis_actor", "Redis Connection ready");
//         RedisActor { conn }
//     }
// }

// #[derive(Message, Debug)]
// #[rtype(result = "Result<Option<String>, redis::RedisError>")]
// struct InfoCommand;

// impl Actor for RedisActor {
//     type Context = Context<Self>;
// }

// impl Handler<InfoCommand> for RedisActor {
//     type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

//     fn handle(&mut self, _msg: InfoCommand, _: &mut Self::Context) -> Self::Result {
//         let mut con = self.conn.clone();
//         let cmd = redis::cmd("INFO");
//         let fut = async move {
//             info!(target: "info_command", "Calling info command");
//             cmd
//                 .query_async(&mut con)
//                 .await
//         };
//         Box::pin(fut)
//     }
// }

// async fn info(redis: web::Data<Addr<RedisActor>>) -> impl Responder {
//     info!(target: "info_request", "sending info command");
//     let res = redis.send(InfoCommand).await.unwrap().unwrap().unwrap();
//     info!(target: "info_request", "Got response {:?}", res);
//     HttpResponse::Ok().body(res)
// }
