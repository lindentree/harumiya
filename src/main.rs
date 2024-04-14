mod create_world_controller;

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use create_world_controller::create_world_simple;
use serde_json::Map;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/create", post(create_world_simple_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, router).await.unwrap();
}
async fn create_world_simple_handler(payload: Json<Map<String, Value>>) -> impl IntoResponse {
    let premise: String = payload.0.get("premise").unwrap().to_string();
    print!("{:?}", premise);
    let result = create_world_simple(payload.0.get("premise").unwrap().to_string()).await;
    print!("{:?}", result);
    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello, World!")
}
