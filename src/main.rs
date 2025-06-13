mod nodes;
mod db;
use std::{sync::Arc, time::Duration};
mod query_builder;
use axum::{
    extract::State, response::Json, routing::get,  Router
};
use serde_json::{Value, json};
use tokio::sync::Mutex;
use crate::db::{try_to_connect, update_nodes};
use db::AppState;
const PORT : &str = "3000";
const CONN_STRING : &str = "host=localhost user=admin password=admin123 dbname=nodesdb port=5000";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared_state = Arc::new(AppState {client : Arc::new(Mutex::new(None))});
    db::try_to_connect(CONN_STRING.to_owned(), shared_state.clone()).await;
    println!("Fetching nodes on server startup");
    {
        //it's 
        let _ = update_nodes(shared_state.clone()).await;
    }
    let st_clone = shared_state.clone();
    tokio::spawn(async move {
        match update_every_duration(st_clone.clone(), Duration::from_secs(60)).await {
            Ok(_) => (),
            Err(error ) => {
                eprintln!("Failed to update, error: {}", error.to_string());
                tokio::spawn(try_to_connect(CONN_STRING.to_owned(), st_clone.clone()));
            }

        };
    });
    let st_clone = shared_state.clone();
    let app : Router<()> = Router::new().route("/nodes", get(serve_nodes_json).with_state(st_clone));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();  


    return Ok(());
    

}
async fn serve_nodes_json (State(state): State<Arc<AppState>>) -> Json<Value> {
    let guard = state.client.lock().await;
    let client = guard.as_ref().unwrap();
    match db::get_nodes_in_db(client).await {
        Ok(nodes_vec) => return Json(json!(nodes_vec)),
        Err(e) => {
            //tries to connect again
            tokio::spawn(try_to_connect(CONN_STRING.to_owned(), state.clone()));
            return Json(json!({
                "code" : 500,
                "error" : e.to_string()
            }))
        } 
    }
 }

async fn update_every_duration(shared_state : Arc<AppState>, duration : Duration) -> Result<(), Box<dyn std::error::Error>>{
    let mut interval = tokio::time::interval(duration);
    //first tick so it waits the duration before ticking again
    interval.tick().await;
    loop {
        interval.tick().await;
        update_nodes(shared_state.clone()).await?;
    }
}


