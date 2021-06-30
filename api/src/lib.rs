pub mod api;
pub(crate) mod test;
pub mod context;
pub mod metrics;
pub mod prelude;
pub mod actors;
pub mod middleware;
pub mod auth;
pub mod util;
pub mod error;
pub mod handlers;
pub mod models;

pub use api_db::db;
pub use api::Api;
pub use error::{ApiError, ApiResult};

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    log::info!("Starting up https://api.devisa.io...");
    Api::new().await.unwrap()
        .run().await.unwrap();
    Ok(())
}

