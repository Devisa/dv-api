use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDbClient, DynamoDb, ListTablesInput, Put, Get, GetItemInput, PutItemInput, Delete, DeleteItemInput
};
pub struct DynamoClient {
    pub region: Region,
    pub client: DynamoDbClient,
}

impl DynamoClient {

    pub async fn new() -> Self {
        let region = Region::UsWest2;
        let client = DynamoDbClient::new(region.clone());
        Self { client, region }
    }

    pub async fn tables(self) -> anyhow::Result<()> {
        let list_tables_request = ListTablesInput::default();
        let tables = self.client.list_tables(list_tables_request).await?;
        println!("Tables found: {:#?}", tables);
        Ok(())
    }
}
