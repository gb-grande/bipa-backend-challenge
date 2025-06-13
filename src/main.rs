mod nodes;
mod db;
use std::{sync::Arc, time::Duration};
mod query_builder;
use axum::{
    extract::State,
    routing::get,
    response::Json,
    Router
};
use serde_json::{Value, json};
use tokio_postgres::{
    Client
};

use crate::db::update_nodes;
struct AppState {
    client: Client

}

const PORT : &str = "3000";
const CONN_STRING : &str = "host=localhost user=admin password=admin123 dbname=nodesdb port=5000";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = db::try_to_connect(CONN_STRING).await;
    let shared_state = Arc::new(AppState { client : client});
    println!("Fetching nodes on server startup");
    update_nodes(&shared_state.client).await?;
    let st_clone = shared_state.clone();
    tokio::spawn(async move {
        update_every_duration(&st_clone.client, Duration::from_secs(60)).await;
    });
    let app : Router<()> = Router::new().route("/nodes", get(serve_nodes_json)).with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();


    return Ok(());
    

}

async fn serve_nodes_json (State(state): State<Arc<AppState>>) -> Json<Value> {
    match db::get_nodes_in_db(&state.client).await {
        Ok(nodes_vec) => return Json(json!(nodes_vec)),
        Err(e) => return Json(json!({
            "code" : 500,
            "error" : e.to_string()

        }))
    }
 }

async fn update_every_duration(client : &Client, duration : Duration) -> Result<(), Box<dyn std::error::Error>>{
    let mut interval = tokio::time::interval(duration);
    //first tick so it waits the duration before ticking again
    interval.tick().await;
    loop {
        interval.tick().await;
        update_nodes(client).await?;
    }
}


