use crate::contents::File;
use crate::vector_db::errors::EmbeddingError;

use dotenv;
use qdrant_client::{
    prelude::{Payload, QdrantClient},
    qdrant::{
        vectors_config::Config, with_payload_selector::SelectorOptions, CreateCollection, Distance,
        PointStruct, ScoredPoint, SearchPoints, VectorParams, VectorsConfig, WithPayloadSelector,
    },
};

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

    pub async fn reset_collection(&self) -> Result<(), EmbeddingError> {
        self.client.delete_collection(COLLECTION).await?;

        self.client
            .create_collection(&CreateCollection {
                collection_name: COLLECTION.to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 768,
                        distance: Distance::Cosine.into(),
                        hnsw_config: None,
                        quantization_config: None,
                        on_disk: None,
                        datatype: None, // Add the datatype field
                    })),
                }),
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    pub async fn upsert_embedding(
        &mut self,
        embedding: Vec<f32>,
        file: &File,
    ) -> Result<(), EmbeddingError> {
        println!("Embedding: {:?}", embedding);
        let payload: Payload = json!({
            "id": file.path.clone(),
        })
        .try_into()
        .unwrap();
        //.map_err(|_| EmbeddingError {})?;

        println!("PAYLOAD: {:?}", payload);

        println!("Embedded: {}", file.path);

        let points = vec![PointStruct::new(self.id, embedding, payload)];
        println!("POINTS: {:?}", points);
        self.client
            .upsert_points(COLLECTION, None, points, None)
            .await?;
        self.id += 1;

        Ok(())
    }

    // pub async fn search(&self, embedding: Embedding) -> Result<ScoredPoint> {
    //     let vec: Vec<f32> = embedding.vec.iter().map(|&x| x as f32).collect();

    //     let payload_selector = WithPayloadSelector {
    //         selector_options: Some(SelectorOptions::Enable(true)),
    //     };

    //     let search_points = SearchPoints {
    //         collection_name: COLLECTION.to_string(),
    //         vector: vec,
    //         limit: 1,
    //         with_payload: Some(payload_selector),
    //         ..Default::default()
    //     };

    //     let search_result = self.client.search_points(&search_points).await?;
    //     let result = search_result.result[0].clone();
    //     Ok(result)
    // }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use qdrant_client::{
        client::QdrantClient,
        qdrant::{vectors_config::Config, CreateCollection, Distance, VectorParams, VectorsConfig},
    };

    use crate::vector_db::{contents::File, errors::EmbeddingError, vector::VectorDB};

    #[tokio::test]
    async fn test_ping_qdrant() {
        dotenv::dotenv().ok(); // Load the .env file
                               // Create a mock QdrantClient
        let client = QdrantClient::from_url(&std::env::var("LOCAL_QDRANT").unwrap()).build();
        let vector_db = VectorDB::new(client.expect("Failed to create Qdrant client"));

        let collections_list = vector_db.client.list_collections().await.unwrap();
        println!("Collections: {:?}", collections_list);

        let response = vector_db.client.list_collections().await;
        assert!(response.is_ok());
    }

    #[tokio::test]

    async fn test_upsert_embedding() {
        dotenv::dotenv().ok(); // Load the .env file
                               // Create a mock QdrantClient
        let client = QdrantClient::from_url(&std::env::var("LOCAL_QDRANT").unwrap()).build();
        let mut vector_db = VectorDB::new(client.expect("Failed to create Qdrant client"));

        // Embed a mock file
        let embedding = vec![1.0, 2.0, 3.0];
        let file = File::new("test".to_string(), "testing".to_string());
        vector_db
            .client
            .create_collection(&CreateCollection {
                collection_name: "docs".to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 100,
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await
            .unwrap();

        vector_db.upsert_embedding(embedding, &file).await.unwrap();

        // Assert that the id has been incremented
        assert_eq!(vector_db.id, 1);
        // vector_db
        //     .client
        //     .delete_collection("test")
        //     .await
        //     .expect("Failed to delete collection");
    }
}
