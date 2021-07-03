use crate::{
    models::session::ApiSession,
    context::{ApiConfig, Context},
    handlers::{self, graphql::{SubscriptionRoot, QueryRoot, MySchema}},
    middleware::cors::builder::Cors};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use actix_session::CookieSession;
use async_graphql::{EmptyMutation, Schema, };
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
        let port = self.ctx.config.port;
        let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot)
            .data(self.ctx.db.pool.clone())
            // .data(Storage::default())
            .finish();
        tracing::info!("GraphQL Playground running on localhost:{}.", &port);
        let _guard = super::metrics::sentry::sentry_opts();
        let enable_redis = std::env::var("ENABLE_REDIS").is_ok();
        tracing::debug!("Running server on port {}", &port);
        let server = HttpServer::new(move || {
            App::new()
                .wrap(Condition::new(true, CookieSession::signed(&[0; 32])
                        .path("/")
                        .domain("devisa.io")
                        .same_site(actix_web::cookie::SameSite::None)
                        .name("dvsa-cookie-sess")
                        .secure(false)
                        .max_age_time(time::Duration::seconds(3600))
                ))
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

    #[actix_rt::test]
    async fn health_check() -> actix_web::Result<()> {
        let mut app = test::init_service(
            App::new().service(handlers::dict)
        ).await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&api_common::models::User::default())
            .to_request();
        let resp = app.call(req).await.unwrap();
        let body = resp.response().body();
        // assert!(body.is_some());
        Ok(())
    }

    #[actix_rt::test]
    async fn index_check() -> actix_web::Result<()> {
        assert_eq!(3, 2);
        Ok(())
    }
}
