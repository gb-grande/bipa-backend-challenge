mod nodes;
use std::sync::Arc;

use axum::{
    extract::State,
    routing::get,
    response::Json,
    Router
};
use serde_json::{Value, json};

struct AppState {
    nodes_vec : Vec<nodes::Node>
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let teste = nodes::get_nodes().await?;
    let nodes = nodes::build_nodes_vec(teste);
    let shared_state = Arc::new(AppState {nodes_vec: nodes});
    let app : Router<()> = Router::new().route("/nodes", get(serve_nodes_json)).with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


    return Ok(());
    

}

async fn serve_nodes_json (State(state): State<Arc<AppState>>) -> Json<Value> {
    let t = &state.nodes_vec;
    Json(json!(t))

}