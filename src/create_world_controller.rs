use std::env;

use dotenv::dotenv;

use gcp_auth::AuthenticationManager;
use harumiya::{Content, GenerateContentRequest, GenerateContentResponse, GenerationConfig, Part};

static MODEL_NAME: &str = "gemini-pro-vision";

pub async fn create_world_simple(user_premise: String) -> Result<(), Box<dyn std::error::Error>> {
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

    let prompt = format!(
        "System: You are a worldbuilding assistant. Create a setting for a novel with the user's premise. Focus more on
        the world's details and less on the plot.
        Format it as a JSON object.

        User: {user_premise}"
    );

    let payload = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part::Text(prompt.to_string())],
        }],
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(2048),
            temperature: Some(0.4),
            top_p: Some(1.0),
            top_k: Some(32),
            ..Default::default()
        }),
        tools: None,
    };

    let resp = reqwest::Client::new()
        .post(&endpoint_url)
        .bearer_auth(token.as_str())
        .json(&payload)
        .send()
        .await?;

    let response = resp.json::<GenerateContentResponse>().await?;
    response.candidates.iter().for_each(|candidate| {
        candidate.content.parts.iter().for_each(|part| {
            if let Part::Text(text) = part {
                print!("{}", text);
            }
        });
    });

    Ok(())
}