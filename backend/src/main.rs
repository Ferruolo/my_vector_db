use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use crate::database_core::VectorDBCore;

mod database_core;
mod ml_interface;
mod vector_b_tree;

const EMBEDDING_PATH: &str = "./embedding.pt";

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
    let vector_db = Arc::new(Mutex::new(VectorDBCore::new(EMBEDDING_PATH)?));
    let app_state = AppState { vector_db };

    let app = Router::new()
        .route("/add_item", post(add_item))
        // .route("/remove_item", post(remove_item))
        .route("/find_top_k_query", post(find_top_k_query))
        .with_state(app_state);

    println!("Server running on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn add_item(
    State(state): State<AppState>,
    Json(payload): Json<AddItemRequest>,
) -> Json<String> {
    let mut db = state.vector_db.lock().await;
    match db.add_item(&payload.item) {
        Ok(_) => Json("Item added successfully".to_string()),
        Err(e) => Json(format!("Error adding item: {}", e)),
    }
}

async fn remove_item(
    State(state): State<AppState>,
    Json(payload): Json<RemoveItemRequest>,
) -> Json<String> {
    let mut db = state.vector_db.lock().await;
    match db.remove_item(&payload.item) {
        Ok(_) => Json("Item removed successfully".to_string()),
        Err(e) => Json(format!("Error removing item: {}", e)),
    }
}

async fn find_top_k_query(
    State(state): State<AppState>,
    Json(payload): Json<FindTopKRequest>,
) -> Json<TopKResponse> {
    let db = state.vector_db.lock().await;
    let results = db.find_k_neighbors(&payload.query, payload.k);
    Json(TopKResponse { results })
}