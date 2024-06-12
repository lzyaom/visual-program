use mongodb::Database;
use std::sync::Arc;
mod init;

pub use init::init_mongo_client;

pub type ShareDB = Arc<Database>;
