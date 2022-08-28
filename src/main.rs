use axum::{routing::get, Router};
use dotenv;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;
use std::net::SocketAddr;

//local modules
mod routes;
mod shared;
// use crate::routes::sign_up;
// use sign_up::sign_up;

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

    //mount the application routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/auth/sign-up", get(routes::sign_up));

    //mount the server
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8052);
    let ip_address = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Ignition started on http://{}", &ip_address);

    axum::Server::bind(&ip_address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
