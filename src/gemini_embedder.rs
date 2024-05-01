use dotenv::dotenv;
use reqwest::header;
use std::env;

use gcp_auth::AuthenticationManager;
use harumiya::{
    Content, EmbeddingContent, EmbeddingsRequest, EmbeddingsResponse, PredictEmbeddingsRequest,
    PredictEmbeddingsResponse,
};

//static EMBEDDING_MODEL_NAME: &str = "embedding-001";
static EMBEDDING_MODEL_NAME: &str = "textembedding-gecko@003";
static EMBEDDING_MODEL_NAME_2: &str = "embedding-001";

pub async fn generate_sentence_embeddings(
    sentence: String,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.

    let endpoint_url = format!(
        "https://{api_endpoint}/v1/projects/{project_id}/locations/{location_id}/publishers/google/models/{EMBEDDING_MODEL_NAME}:predict"
    );

    //print!("EMBED {:?}", EMBEDDING_MODEL_NAME);
    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    // let sample_text = r#"Things to Consider When Designing a Planet
    // Distance from star: The distance a planet orbits its parent star determines how much solar heat and radiation it receives.
    // For bright, hot stars, the habitable zone is farther away than for cool, dim stars.
    // "#;

    //let sample = "The distance a planet orbits its parent star determines how much solar heat";

    let payload = PredictEmbeddingsRequest {
        instances: vec![EmbeddingContent { content: sentence }],
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

    //println!("EMBED TEST{:?}", response);

    Ok(response.predictions[0].embeddings.values.clone())
}

// pub async fn generate_embeddings_v2() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv().ok();

//     let api_endpoint = env::var("API_ENDPOINT2")?;
//     // let project_id = env::var("PROJECT_ID")?;
//     // let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.
//     let key = env::var("API_KEY")?;

//     let endpoint_url = format!(
//         "https://{api_endpoint}/v1beta/models/{EMBEDDING_MODEL_NAME_2}:embedContent&apiKey={key}"
//     );

//     print!("SEC {:?}", EMBEDDING_MODEL_NAME_2);
//     let authentication_manager = AuthenticationManager::new().await?;
//     let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
//     let token = authentication_manager.get_token(scopes).await?;

//     let sample_text = r#"Things to Consider When Designing a Planet
//     Distance from star: The distance a planet orbits its parent star determines how much solar heat and radiation it receives.
//     For bright, hot stars, the habitable zone is farther away than for cool, dim stars.
//     "#;

//     let payload = EmbeddingsRequest {
//         content: Content {
//             role: "system".to_string(),
//             parts: vec![harumiya::Part::Text(sample_text.to_string())],
//         },
//     };

//     print!("{:?}", payload);

//     let resp = reqwest::Client::new()
//         .post(&endpoint_url)
//         .header(header::CONTENT_TYPE, "application/json")
//         .header(header::AUTHORIZATION, key)
//         .bearer_auth(token.as_str())
//         .json(&payload)
//         .send()
//         .await?;

//     let resp = resp.bytes().await?;
//     println!("V2 RES {:#?}", resp);

//     // let response = resp.json::<EmbeddingsResponse>().await?;

//     // println!("V2 TEST{:?}", response);

//     Ok(())
// }

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_sentence_embeddings() {
        let result = generate_sentence_embeddings(
            "The distance a planet orbits its parent star determines how much solar heat"
                .to_string(),
        )
        .await;
        print!("{:?}", result);
        assert!(result.is_ok());
    }

    // #[tokio::test]
    // async fn test_generate_embeddings_v2() {
    //     let result = generate_embeddings_v2().await;
    //     print!("{:?}", result);
    //     assert!(result.is_ok());
    // }
}
