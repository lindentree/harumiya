mod create_world_controller;

use async_stream::stream;
use axum::body::Body;
use axum::http::Error;
use axum::response::sse::{self, Event};
use axum::response::sse::{Event as SseEvent, KeepAlive, Sse};
use axum::response::Response;
use axum::BoxError;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use create_world_controller::create_world_simple;
use futures::stream::Stream;
use futures::stream::{self, StreamExt};
use futures::TryStreamExt;
use harumiya::GenerateContentResponse;
use serde_json::Map;
use serde_json::Value;
use std::convert::Infallible;
use std::f32::consts::E;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::time::Duration;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/create", post(create_world_simple_handler));
    // .route("/sse", post(create_world_sse));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
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
            .body(Body::from(_result))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Error"))
            .unwrap(),
    }
}

// async fn create_world_sse(
//     payload: Json<Map<String, Value>>,
// ) -> sse::Sse<impl Stream<Item = Result<Event, BoxError>> + Send + 'static>{
//     let premise: String = payload.0.get("premise").unwrap().to_string();
//     print!("{:?}", premise);

//     let stream = stream! {
//         let result = create_world_simple(payload.0.get("premise").unwrap().to_string().to_owned()).await;
//         print!("{:?}", result);
//         match result {
//             Ok(_result) => {
//                 yield Ok(SseEvent::default().data("Success"));
//             }
//             Err(_) => {
//                 yield Ok(SseEvent::default().data("Error"));
//             }
//         }
//     };

//     Sse::new(stream)
// }

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}
