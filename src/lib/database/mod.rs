pub mod collections;
use mongodb::{Client, Database};

use std::env;
use std::error::Error;

pub async fn get_db() -> Result<Database, Box<dyn Error>> {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let client = Client::with_uri_str(client_uri.as_ref()).await?;

    return Ok(client.database("discord"));
}
