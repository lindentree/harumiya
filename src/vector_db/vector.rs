use crate::vector_db::contents::File;

use crate::vector_db::errors::EmbeddingError;
use qdrant_client::{client::Payload, prelude::QdrantClient, qdrant::PointStruct};
use serde_json::json;

const COLLECTION: &str = "docs";
pub struct VectorDB {
    client: QdrantClient,
    id: u64,
}

impl VectorDB {
    pub fn new(client: QdrantClient) -> Self {
        Self { client, id: 0 }
    }

    pub async fn upsert_embedding(
        &mut self,
        embedding: Vec<f32>,
        file: &File,
    ) -> Result<(), EmbeddingError> {
        let payload: Payload = json!({
            "id": file.path.clone(),
        })
        .try_into()
        .map_err(|_| EmbeddingError {})?;

        println!("Embedded: {}", file.path);

        //let vec: Vec<f64> = embedding.vec.iter().map(|&x| x as f64).collect();

        let points = vec![PointStruct::new(self.id, embedding, payload)];
        self.client
            .upsert_points(COLLECTION, None, points, None)
            .await?;
        self.id += 1;

        Ok(())
    }
}
