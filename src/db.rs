use tokio_postgres::{
    Client, NoTls
};
use std::{time::Duration};
use crate::nodes;
use crate::nodes::Node;
use crate::query_builder;
use std::sync::{Arc};
use tokio::sync::Mutex;
use serde_json::Value;
//tries to connect to db client, 

pub struct AppState {
    pub client: Arc<Mutex<Option<Client>>>
}


pub async fn try_to_connect(conn_string : String, shared_state : Arc<AppState>) -> (){
    println!("Trying to connect to db");
    //reconection loop
    loop {
        match tokio_postgres::connect(conn_string.as_str(), NoTls).await{
            Ok((client, connection)) =>{
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }

                });
                //update app state with client
                *shared_state.client.lock().await = Some(client);
                println!("Connection with db estabilished");
                return ;
                }

            Err(e) => {
                println!("Error {}", e);
                println!("Connection failed, retrying in 5 seconds");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }    
}

//update nodes table upon receiving appstate
//only lock after fetching nodes
pub async fn update_nodes(appstate : Arc<AppState>) -> Result<(), Box<dyn std::error::Error>>{
    println!("Updating nodes");
    let json_vec : Vec<Value>;
    //fetches from api
    loop {
        match nodes::get_nodes().await {
            Ok(vec) => {
                json_vec = vec;
                break;
            }
            Err(e) =>{
                println!("Error {} while trying to fetch, will retry in 10s", e.to_string());
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
 
        };

    }
    //converts to node struct
    let nodes_vec = nodes::Node::build_nodes_vec_from_json(json_vec);
    //builds query
    let query = query_builder::UpdateDbQuery::build_from_nodes(&nodes_vec);
    let query_str = query.get_querry();
    appstate.client.lock().await.as_ref().unwrap().simple_query(query_str).await?;
    println!("Succesful update");
    return Ok(());
}


pub async fn get_nodes_in_db(client : &Client) -> Result<Vec<Node>, Box<dyn std::error::Error>>{
    let rows = client.query("SELECT * FROM nodes;", &[]).await?;
    let nodes_vec = Node::build_nodes_vec_from_rows(&rows);
    return Ok(nodes_vec);
    
}

