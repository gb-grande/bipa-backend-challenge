use tokio_postgres::{
    Client, Error, NoTls
};
use std::time::Duration;
use crate::nodes;
use crate::nodes::Node;
use crate::query_builder;
//tries to connect to db client, 
pub async fn try_to_connect(conn_string : &str) -> Client{
    loop {
        match tokio_postgres::connect(conn_string, NoTls).await{
            Ok((client, connection)) =>{
                tokio::spawn(async move {
                    if let Err(e) = connection.await {
                        eprintln!("connection error: {}", e);
                    }
                });
                return client;
            }
            Err(e) => {
                println!("Error {}", e);
                println!("Connection failed, retrying in 5 seconds");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }    
}

pub async fn is_nodes_table_empty(client : &Client) -> Result<bool, Error>{
    let rows = client.query("SELECT * FROM NODES;", &[]).await?;
    if rows.len() == 0 {
        return Ok(true);
    }
    return Ok(false);
}
//update nodes table
pub async fn update_nodes(client : &Client) -> Result<(), Box<dyn std::error::Error>>{
    println!("Updating nodes");
    //fetches from api
    let json_vec = nodes::get_nodes().await?;
    //converts to node struct
    let nodes_vec = nodes::Node::build_nodes_vec_from_json(json_vec);
    //builds query
    let query = query_builder::UpdateDbQuery::build_from_nodes(&nodes_vec);
    let query_str = query.get_querry();
    client.simple_query(query_str).await?;
    println!("Succesful update");
    return Ok(());
}

pub async fn get_nodes_in_db(client : &Client) -> Result<(Vec<Node>), Box<dyn std::error::Error>>{
    let rows = client.query("SELECT * FROM nodes;", &[]).await?;
    let nodes_vec = Node::build_nodes_vec_from_rows(&rows);
    return Ok(nodes_vec);
    
}