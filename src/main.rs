use axum::{routing::get, Router};
use log::{debug, info};
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read in env vars
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()?;

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;

    let tcp = TcpListener::bind(&addr).await?;

    let router = Router::new().route("/", get(handle_request));

    println!("Server is running on {}:{}", host, port);

    let server_handle = axum::serve(tcp, router).await;

    if let Err(e) = server_handle {
        eprintln!("Server error: {}", e);
    }

    Ok(())
}

async fn handle_request() -> &'static str {
    info!("Client Connected"); // server log
    "Connection was successful" // answer for client
}
