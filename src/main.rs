use dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    //try reading the environment variables
    dotenv::dotenv().expect("Failed to read .env file");
    let database_uri = env::var("DATABASE_URI").expect("error reading database URI");
    let database_name = env::var("DATABASE_NAME").expect("error reading database name");

    // Get a handle to the cluster  and Ping the server to see if you can connect to the cluster
    let database_client_options = ClientOptions::parse(&database_uri).await?;
    let database_client = Client::with_options(database_client_options)?;
    database_client
        .database(&database_name)
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Successfully Connected to Database.");

    Ok(())
}
