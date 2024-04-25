use axum::{body, Json};
use dotenv::dotenv;
use reqwest::header;
use std::{env, f32::consts::E};

use gcp_auth::AuthenticationManager;
use harumiya::{
    Content, EmbedPart, EmbeddingContent, GenerationEmbedConfig, Part, PredictEmbeddingsRequest,
    PredictEmbeddingsResponse,
};

//static EMBEDDING_MODEL_NAME: &str = "text-embedding-004";
//static EMBEDDING_MODEL_NAME: &str = "embedding-001";
static EMBEDDING_MODEL_NAME: &str = "textembedding-gecko@003";

pub async fn generate_embeddings() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.

    let endpoint_url = format!(
        "https://{api_endpoint}/v1/projects/{project_id}/locations/{location_id}/publishers/google/models/{EMBEDDING_MODEL_NAME}:predict"
    );

    print!("EMBED {:?}", EMBEDDING_MODEL_NAME);
    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    // let sample_text = r#"Things to Consider When Designing a Planet
    // Distance from star: The distance a planet orbits its parent star determines how much solar heat and radiation it receives.
    // For bright, hot stars, the habitable zone is farther away than for cool, dim stars.
    // "#;

    let sample = "The distance a planet orbits its parent star determines how much solar heat";

    let payload = PredictEmbeddingsRequest {
        instances: vec![EmbeddingContent {
            content: sample.to_string(),
        }],
    };

    print!("{:?}", payload);

    let resp = reqwest::Client::new()
        .post(&endpoint_url)
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(token.as_str())
        .json(&payload)
        .send()
        .await?;

    // let resp = resp.bytes().await?;
    // println!("{:#?}", resp);

    let response = resp.json::<PredictEmbeddingsResponse>().await?;

    println!("EMBED TEST{:?}", response);

    Ok(())
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_embeddings() {
        let result = generate_embeddings().await;
        print!("{:?}", result);
        assert!(result.is_ok());
    }
}
