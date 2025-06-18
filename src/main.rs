mod api_handler;

use std::net::SocketAddr;
use dotenv::dotenv;

use axum::{routing::{get, post}, Router};

#[tokio::main]
async fn main() {
    println!("Signing Service!");
     // Load variables from .env file
    //  dotenv().ok();
    dotenv().ok(); 

    tracing_subscriber::fmt()
        .init();

    let app = Router::new()
        .route("/", get(|| async { "API is live!" }))
        .route("/sign-doc", post(api_handler::sign_doc));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> Listening on {addr}\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
