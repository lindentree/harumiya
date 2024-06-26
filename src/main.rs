mod create_world_controller;
mod gemini_embedder;
mod vector_db;

use crate::vector_db::vector::VectorDB;
use crate::vector_db::{contents, gemini};
use axum::body::Body;
use axum::response::Response;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use axum::{Json, Router};
use create_world_controller::create_world_simple;
use dotenv::dotenv;
use qdrant_client::{
    prelude::{Payload, QdrantClient},
    qdrant::{
        vectors_config::Config, with_payload_selector::SelectorOptions, CreateCollection, Distance,
        PointStruct, ScoredPoint, SearchPoints, VectorParams, VectorsConfig, WithPayloadSelector,
    },
};
use serde_json::Map;
use serde_json::Value;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use vector_db::contents::File; // Add this import statement // Add this import statement

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    dotenv().ok();

    // let qdrant_client = QdrantClient::from_url(&std::env::var("LOCAL_QDRANT").unwrap()).build();
    // let mut vector_db = VectorDB::new(qdrant_client.expect("Failed to create Qdrant client"));
    // let files =
    //     contents::load_txt_files_from_dir("./documents".into(), ".txt", &".".into()).unwrap();
    // println!("Files: {:?}", files.len());

    // vector_db.reset_collection().await.unwrap();

    // println!("Setup done");

    // embed_documentation(&mut vector_db, &files).await.unwrap();
    // println!("Embedding done");

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/create", post(create_world_simple_handler))
        .route("/create-detailed", post(create_world_detailed_handler));

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

async fn create_world_detailed_handler(payload: Json<Map<String, Value>>) -> impl IntoResponse {
    let premise: String = payload.0.get("premise").unwrap().to_string();
    let result = create_world_controller::create_world_detailed(Json(premise)).await;
    println!("DETAIL outer: {:?}", result);

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

// async fn embed_documentation(
//     vector_db: &mut VectorDB,
//     files: &Vec<File>,
// ) -> anyhow::Result<(), Box<dyn std::error::Error>> {
//     for file in files {
//         let embeddings = gemini::embed_file(file).await?;
//         println!("Embedding: {:?}", file.path);
//         println!("CHECK: {:?}", embeddings.len());
//         for embedding in embeddings {
//             vector_db.upsert_embedding(embedding, file).await?;
//         }
//     }

//     Ok(())
// }

pub async fn hello_world() -> &'static str {
    "Hello, world change!"
}
