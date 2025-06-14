use reqwest::Error;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use chrono:: {DateTime, Utc};
use tokio_postgres::Row;
const BTC_SATS_RATIO : i32 = 100000000; 
const FETCH_URL : &str = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

//Structure which represents a node in memory
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub public_key : String,
    pub alias : String,
    pub capacity : f64,
    pub first_seen : DateTime<Utc>
}


//get nodes from api
pub async fn get_nodes() -> Result<Vec<Value>, Error> {
    let response = reqwest::get(FETCH_URL).await?.json::<Vec<serde_json::Value>>().await?;
    return Ok(response);
}
impl Node {
    //receives json and converts it to Node struct
    pub fn build_from_json(json : &Value) -> Node {
        let capacity_in_sats = json["capacity"].as_i64();
        let capacity_in_btc : f64;
        //if for any reason any of the fields is, it will be replaced by a default value - empty string or epoch as a date
        match capacity_in_sats {
            Some(num) => capacity_in_btc = num as f64/BTC_SATS_RATIO as f64,
            None => capacity_in_btc = 0f64
        }
        let first_seen_in_seconds : i64 = match json["firstSeen"].as_i64() {
            Some(t) => t ,
            None => 0
        };
        let first_seen_as_date :  DateTime<Utc> =  match DateTime::from_timestamp(first_seen_in_seconds, 0) {
            Some(date) => date,
            None => DateTime::from_timestamp(0, 0).unwrap()
        };
        let alias : String = match json["alias"].as_str() {
            Some(string) => string.to_string(),
            None => String::new()

        };
        let public_key : String = match json["publicKey"].as_str() {
            Some(string) => string.to_string(),
            None => String::new()

        };
        let new_node = Node {
            capacity : capacity_in_btc,
            public_key : public_key,
            alias : alias,
            first_seen : first_seen_as_date
        };
        return new_node;
    }
    
    //converts a json Vector to a node struct Vector
    pub fn build_nodes_vec_from_json(vec_json : Vec<Value>) -> Vec<Node> {
        let mut node_vec : Vec<Node> = Vec::new();
        for json in vec_json.iter() {
            node_vec.push(Node::build_from_json(json));
        }
        return node_vec;

    }
    //receives a bd row and creates the corresponding Node structure
    pub fn build_from_row(row : &Row) -> Self {
        let alias : String = row.get("alias");
        let capacity : f64 = row.get("capacity");
        let first_seen : DateTime<Utc> = row.get("first_seen");
        let public_key : String = row.get("public_key");
        return Self {
            public_key : public_key,
            capacity : capacity,
            alias: alias,
            first_seen : first_seen
        };

    }
    //same as before but receives an array
    pub fn build_nodes_vec_from_rows(rows : &Vec<Row>) -> Vec<Self> {
        let mut nodes_vec : Vec<Node> = Vec::<Node>::new();
        for row in rows.iter() {
            nodes_vec.push(Node::build_from_row(row));
        }
        return nodes_vec;
    }
    

}