use std::env;

use axum::Json;
use dotenv::dotenv;

use gcp_auth::AuthenticationManager;
use harumiya::{Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part};
use qdrant_client::client::QdrantClient;

use crate::{
    gemini_embedder::{self, generate_sentence_embeddings},
    vector_db::{
        self,
        contents::{self, File},
        errors::EmbeddingError,
        gemini,
        vector::{self, VectorDB},
    },
};

use vector_db::finder::Finder;

static MODEL_NAME: &str = "gemini-pro-vision";
//static MODEL_NAME: &str = "gemini-1.5-pro-latest";

struct AppState {
    files: Vec<File>,
    vector_db: VectorDB,
}

pub async fn create_world_simple(
    user_premise: String,
) -> Result<Json<String>, Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_endpoint = env::var("API_ENDPOINT")?;
    let project_id = env::var("PROJECT_ID")?;
    let location_id = env::var("LOCATION_ID")?; // Sometimes called "region" in gCloud docs.

    let endpoint_url = format!(
        "https://{api_endpoint}/v1/projects/{project_id}/locations/{location_id}/publishers/google/models/{MODEL_NAME}:generateContent"
    );

    println!("RUNNING");

    let authentication_manager = AuthenticationManager::new().await?;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = authentication_manager.get_token(scopes).await?;

    let init_prompt = format!(
        r#"System: You are a worldbuilding assistant. Create a setting based on the user's premise. Focus more on
        the world's details and less on the plot. 
        Format it as a JSON with this schema {{ "name": "multiline string", "setting": "multiline string", "wildlife: "multiline string", "geography": "multiline string", etc  }}. Do not nest data 
        but flesh out each category with a few concrete examples. Also add more categories as appropriate.
        User: {user_premise}"#
    );
    println!("PROMPT {:?}", init_prompt);

    let prompt_embedding = generate_sentence_embeddings(init_prompt).await?;

    let qdrant_client = QdrantClient::from_url(&std::env::var("LOCAL_QDRANT").unwrap()).build();
    let vector_db = VectorDB::new(qdrant_client.expect("Failed to create Qdrant client"));
    let context = vector_db.search(prompt_embedding).await?;
    let files = contents::load_txt_files_from_dir("./documents".into(), ".txt", &".".into())?;

    let result = files.get_contents(&context).ok_or(EmbeddingError)?;

    let final_prompt = format!(
        r#"System: You are a worldbuilding assistant. Create a setting based on the user's premise. Focus more on
        the world's details and less on the plot. 
        Format it as a JSON with this schema {{ "name": "multiline string", "setting": "multiline string", "wildlife: "multiline string", "geography": "multiline string", etc  }}. Do not nest data 
        but flesh out each category with a few concrete examples. Also add more categories as appropriate.
        User: {user_premise}
        Context: {result}"#
    );

    println!("FINAL PROMPT {:?}", final_prompt);

    let payload = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part::Text(final_prompt)],
        }],
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(2048),
            temperature: Some(0.4),
            top_p: Some(1.0),
            top_k: Some(32),
            //response_mime_type: Some("application/json".to_string()),
            ..Default::default()
        }),
        tools: None,
    };

    println!("PROMPT PAYLOAD{:?}", payload);

    let resp = reqwest::Client::new()
        .post(&endpoint_url)
        .bearer_auth(token.as_str())
        .json(&payload)
        .send()
        .await?;

    let mut assembled = String::new();

    let response = resp.json::<GenerateContentResponse>().await?;
    response.candidates.iter().for_each(|candidate| {
        candidate.content.parts.iter().for_each(|part| {
            if let Part::Text(text) = part {
                print!("{}", text);
                assembled += text;
            }
        });
    });

    println!("CONTROLLER {:?}", Json(&assembled));

    Ok(Json(assembled))
}

pub async fn get_context() {}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_world_simple() {
        let result =
            create_world_simple("I want to create a world with dragons.".to_string()).await;
        assert!(result.is_ok());
    }
}
