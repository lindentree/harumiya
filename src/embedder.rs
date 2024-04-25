// use arrow_array::{RecordBatch, RecordBatchIterator};
// use arrow_schema::{DataType, Field, Schema};
// use lancedb::arrow::{arrow_array, arrow_schema};
// use lancedb
// use serde_json::json;

// use lopdf::Document;

// pub fn ingest_pdf() {
//     let mut doc = Document::load("example.pdf").unwrap();
//     let pages = doc.get_pages();
//     for page in pages {
//         let content = doc.get_page_content(page).unwrap();
//         println!("{:?}", content);
//     }
// }

// pub fn generate_embeddings() -> embeddings {
//     let embedding_function = EmbeddingFunction {
//         name: "universal-sentence-encoder".to_string(),
//         version: "4".to_string(),
//     };
//     return;
// }

// pub async fn create_lance_db() {
//     let db = lancedb::connect("data/sample-lancedb")
//         .execute()
//         .await
//         .unwrap();
//     let model = lancedb::get_model("gemini-text").unwrap().create();
// }

// // pub fn create_chroma_db(documents: String, name: String) -> Result<(), Box<dyn std::error::Error>> {
// //     // With default ChromaClientOptions
// //     // Defaults to http://localhost:8000
// //     let client: ChromaClient = ChromaClient::new(Default::default());

// //     let collection: ChromaCollection = client.get_or_create_collection("embeddings", None)?;

// //     let documents = vec![json!({
// //         "name": name,
// //         "content": documents,
// //     })];

// //     collection.add(documents, generate_embeddings())?;

// //     Ok(())
// // }
