mod create_world_controller;
mod gemini_embedder;
mod vector_db;

use crate::vector_db::vector::VectorDB;
use axum::body::Body;
use axum::response::Response;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use axum::{Json, Router};
use create_world_controller::create_world_simple;
use serde_json::Map;
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use vector_db::contents::File; // Add this import statement // Add this import statement

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/create", post(create_world_simple_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let tcp = TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp, router).await.unwrap();
}

async fn create_world_simple_handler(payload: Json<Map<String, Value>>) -> impl IntoResponse {
    let premise: String = payload.0.get("premise").unwrap().to_string();
    let result = create_world_simple(premise).await;
    println!("outer: {:?}", result);

    match result {
        Ok(_result) => Response::builder()
            .status(StatusCode::CREATED)
            .body(Body::from(_result.to_string()))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Error"))
            .unwrap(),
    }
}

// async fn embed_documentation(vector_db: &mut VectorDB, files: &Vec<File>) -> anyhow::Result<()> {
//     for file in files {
//         let embeddings = open_ai::embed_file(file).await?;
//         println!("Embedding: {:?}", file.path);
//         for embedding in embeddings.data {
//             vector_db.upsert_embedding(embedding, file).await?;
//         }
//     }

//     Ok(())
// }

pub async fn hello_world() -> &'static str {
    "Hello, world change!"
}
