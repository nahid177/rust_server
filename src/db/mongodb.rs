use mongodb::{Client, Database};
use dotenv::dotenv;
use std::env;
use mongodb::options::ClientOptions;

pub async fn connect_db() -> Database {
    dotenv().ok(); // Load environment variables

    // Fetch MongoDB URI from environment variables
    let mongo_url = env::var("MONGO_URL").expect("MONGO_URL not set in .env file");

    // Parse client options and establish connection to MongoDB
    let mut client_options = ClientOptions::parse(&mongo_url)
        .await
        .expect("Failed to parse MongoDB URI");

    client_options.app_name = Some("RustBackendApp".to_string());

    let client = Client::with_options(client_options).expect("Failed to connect to MongoDB");

    // Access database specified in environment variables, default to "test"
    let db_name = env::var("DB_NAME").unwrap_or_else(|_| "test".to_string());
    let db = client.database(&db_name);

    println!("Connected to MongoDB at {}", mongo_url);

    db
}
