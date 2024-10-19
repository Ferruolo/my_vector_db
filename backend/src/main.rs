mod helpers;
mod llama_embedding;
mod node_interface;
mod types;
mod vector_db;
mod DB_interface;

use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::database_core::VectorDBCore;

#[derive(Clone)]
struct AppState {
    vector_db: Arc<Mutex<VectorDBCore>>,
}

#[derive(Deserialize)]
struct AddItemRequest {
    item: String,
}

#[derive(Deserialize)]
struct RemoveItemRequest {
    item: String,
}

#[derive(Deserialize)]
struct FindTopKRequest {
    query: String,
    k: usize,
}

#[derive(Serialize)]
struct TopKResponse {
    results: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

}
