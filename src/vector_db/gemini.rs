use crate::gemini_embedder;
use crate::vector_db::contents::File;
use anyhow::Result;

pub async fn embed_file(file: &File) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
    let sentence_as_str: Vec<&str> = file.sentences.iter().map(|s| s.as_str()).collect();
    //println!("Sentences: {:?}", sentence_as_str);
    let mut embeddings = Vec::new();
    for sentence in sentence_as_str {
        let embedding = gemini_embedder::generate_sentence_embeddings(sentence.to_string()).await?;
        embeddings.push(embedding);
        //println!("Embedded: {}", sentence);
    }

    Ok(embeddings)
}

pub async fn embed_sentence(prompt: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let embedding = gemini_embedder::generate_sentence_embeddings(prompt.to_string()).await?;
    Ok(embedding)
}
