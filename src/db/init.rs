use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;

pub async fn init_db() -> Result<(), Box<dyn Error>> {
    // Load MongoDB connection string from .env
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environmental variable!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
  
    // Print dbs in cluster
    println!("Databases");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    Ok(())
}

// https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial