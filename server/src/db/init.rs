use super::ShareDB;
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::Arc;

pub async fn init_mongo_client() -> ShareDB {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("database url must set");

    let mut client_options = ClientOptions::parse(&url).await.unwrap();

    client_options.app_name = Some("Program".to_string());

    let client = Client::with_options(client_options).unwrap();

    let db_name = env::var("DATABASE_NAME").expect("database name must set");

    let share_db = client.database(&db_name);

    Arc::new(share_db)
}
