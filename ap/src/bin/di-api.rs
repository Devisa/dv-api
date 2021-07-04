#[actix_web::main]
async fn main() -> std::io::Result<()> {
    di_api::run().await
}
