use axum::{
    extract::Host,
    http::{header, HeaderMap, Method},
    routing::get,
    Router,
};
use log::{debug, error};
use std::env;
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

    let router = Router::new()
        .route("/", get(handle_request))
        .route("/help", get(handle_help));

    println!("Server is running on {}:{}", host, port);

    let server_handle = axum::serve(tcp, router).await;

    if let Err(e) = server_handle {
        eprintln!("Server error: {}", e);
    }
    Ok(())
}

async fn handle_request(host: Host, method: Method, headers: HeaderMap) -> String {
    println!("Client Connected"); // server log
    println!("Received Request from {}", host.0); //
    println!("{}", method); //
    println!(
        "Host: {}",
        env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
    );

    match headers.get(header::USER_AGENT) {
        Some(user_agent) => match user_agent.to_str() {
            Ok(val) => println!("User Agent: {}", val),
            Err(err) => error!(
                "Error trying to convert user agent header to string: {}",
                err
            ),
        },
        None => println!("User Agent: No user agent header."),
    }
    //println!("Headers: {:?}", headers);

    // forward request
    forward_request().await
}

async fn forward_request() -> String {
    reqwest::get("http://127.0.0.1:5000")
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

async fn handle_help() -> &'static str {
    debug!("Client Connected"); // server log
    "There is no help." // answer for client
}
