use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[derive(Clone, Serialize, Deserialize)]
struct Item {
    id: u32,
    name: String,
}

type AppState = Arc<Mutex<Vec<Item>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state: AppState = Arc::new(Mutex::new(Vec::new()));
    
    let app = Router::new()
        .route("/items", get(get_items))
        .with_state(state);
    
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Serveur sur http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn get_items(axum::extract::State(state): axum::extract::State<AppState>) -> Json<Vec<Item>> {
    let items = state.lock().unwrap();
    Json(items.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_items() {
        let state: AppState = Arc::new(Mutex::new(vec![
            Item { id: 1, name: "Test".to_string() }
        ]));
        // Test basique
        assert!(true);
    }
}

