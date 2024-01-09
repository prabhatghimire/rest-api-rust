// database/mongodb.rs

use lazy_static::lazy_static;
use mongodb::{Client, Database};
use std::sync::Mutex;

lazy_static! {
    static ref MONGO_CLIENT: Mutex<Option<Client>> = Mutex::new(None);
}

pub async fn establish_connection(mongodb_uri: &str) {
    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");
    let mut mongo_client = MONGO_CLIENT.lock().unwrap();
    *mongo_client = Some(client);
}

pub fn get_database() -> Database {
    let mongo_client = MONGO_CLIENT.lock().unwrap();
    mongo_client
        .as_ref()
        .expect("MongoDB client not initialized")
        .database("your_database_name")
}
