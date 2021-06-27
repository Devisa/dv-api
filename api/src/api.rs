use crate::{context::{ApiConfig, ApiSession, Context}, handlers, middleware::cors::builder::Cors};
use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, get, web,
    middleware::{
        Condition, Compress, Logger, NormalizePath
    },
};
use api_db::Db;
use tracing::Level;

#[derive(Debug, Clone)]
pub struct Api {
    pub ctx: Context,
}

impl Api {

    pub async fn new() -> anyhow::Result<Self> {
        match Context::new().await {
            Ok(ctx) => Ok(Self { ctx }),
            Err(e) => Err(anyhow::anyhow!("{}", &e)),
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        std::env::set_var("RUST_LOG", "actix_server=infoo,actix_web=trace,actix_redis=trace");
        /* let env = env_logger::Env::default()
            .filter_or("MY_LOG_LEVEL", "info")
            .write_style_or("MY_LOG_STYLE", "always");
        env_logger::Builder::from_env(env).init(); */
        /* let _collector = tracing_subscriber::fmt()
            .try_init()
            .expect("Could not initialize tracing subscriber"); */

        let _guard = super::metrics::sentry::sentry_opts();
        let enable_redis = std::env::var("NORMALIZE_PATH").is_ok();
        let port = self.ctx.config.port;
        log::debug!("Running server on port {}", &port);
        let server = HttpServer::new(move || {
            App::new()
                // .wrap(Cors::permissive())
                .wrap(Condition::new(true, Cors::permissive()))
                .wrap(Condition::new(true, NormalizePath::default()))
                .wrap(Compress::default())
                .wrap(NormalizePath::default())
                .wrap(Logger::new("\n%r\nSTATUS %s (%Dms, %bb)\nFrom %a to %U\nUser Agent: %{User-Agent}i\nReferrer: %{Referer}i"))
                .service(health)
                .data(ApiSession::default())
                .data(self.ctx.db.clone())
                .data(self.ctx.redis.clone())
                .configure(handlers::routes)
        })
            .bind(format!("0.0.0.0:{}", &port))?
            .run().await?;
        Ok(())
    }
}

/// Health check route
#[get("/health")]
async fn health() -> impl actix_web::Responder {
    tracing::info_span!("Health check succeeded!");
    HttpResponse::Ok().finish()
}



#[cfg(test)]
mod test {
    use super::*;
    use actix_web::{http, body, web, App, test, dev::Service};
    use chrono::Utc;
    // use super::handlers::{EchoObj, echo};

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
            // .filter_level(log::LevelFilter::max())
    }

    /* #[actix_rt::test]
    async fn health_check() -> actix_web::Result<()> {
        init_logger();
        log::info!("HEALTH CHECK TEST. Running...");
        log::debug!("Beginning test.");
        let mut app = test::init_service(
            App::new().service(handlers::echo)
        ).await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&EchoObj {
                data: String::from("health check object"),
                time: Utc::now(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        let body = resp.response().body().as_ref();
        assert!(body.is_some());
        Ok(())
    } */

    // #[actix_rt::test]
    // async fn index_check() -> actix_web::Result<()> {
    //     init_logger();
    //     log::info!("RUNNING index_check TEST NOW...");
    //     log::debug!("Beginning test.");
    //     assert_eq!(3, 2);
    //     Ok(())
    // }
}
