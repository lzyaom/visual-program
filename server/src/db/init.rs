use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn init_mongo_client() -> mongodb::error::Result<Client> {
    let url = env::var("DATABASE_URL").expect("database url must set");

    let mut client_options = ClientOptions::parse(&url).await?;

    client_options.app_name = Some("Program".to_string());

    let client = Client::with_options(client_options)?;

    Ok(client)
}
