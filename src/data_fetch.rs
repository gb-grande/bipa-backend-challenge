use reqwest::Error;
use serde_json::Value;


const FETCH_URL : &str = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

//função que bate no endpoint e retorna um vec de jsons
pub async  fn get_nodes() -> Result<Vec<Value>, Error> {
    let response = reqwest::get(FETCH_URL).await?.json::<Vec<serde_json::Value>>().await?;
    return Ok(response);
}