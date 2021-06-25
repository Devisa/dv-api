use api_cloud::{
    s3::*,
    dynamodb::DynamoClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    DynamoClient::new().await.tables().await?;
    Ok(())
}
