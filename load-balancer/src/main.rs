mod processor;

use axum::{
    extract::{Request, State},
    http::header::HeaderMap,
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use processor::Processor;
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

    let mut processor = Processor::new();

    processor.register_servers(&mut vec![
        "http://127.0.0.1:5000".to_string(),
        "http://127.0.0.1:5001".to_string(),
    ]);

    let router = Router::new()
        .route("/", get(handle_request))
        .with_state(processor);

    println!("Server is running on {}:{}", host, port);

    let server_handle = axum::serve(tcp, router).await;

    if let Err(e) = server_handle {
        eprintln!("Server error: {}", e);
    }
    Ok(())
}

#[debug_handler]
pub async fn handle_request(
    State(state): State<Processor>,
    header_map: HeaderMap,
    request: Request,
) -> impl IntoResponse {
    println!("Client Connected");
    println!("Request: {:?}", request);
    println!("Headers: {:?}", header_map);
    println!(
        "Host: {}",
        env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string())
    );

    let current_server;
    let current_request;

    {
        // forward request
        let mut last_request = state.last_request.lock().unwrap();
        let servers = state.servers.lock().unwrap();

        current_request = (*last_request + 1) % servers.len();

        if let Some(server) = servers.get(current_request) {
            println!("Senc request to server: {}", server);
            current_server = server.clone();
        } else {
            println!("No servers available");
            return "Unfortunately there are no servers available at the moment :(".to_string();
        }

        *last_request = current_request;
    }

    reqwest::get(current_server.to_string())
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
