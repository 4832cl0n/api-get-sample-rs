extern crate reqwest;
extern crate tokio;
use reqwest::{Client, Url};

const DEV_URI: &str = "https://bitflyer.com";
const URI: &str = "https://bitflyer.com";
const PRICE: &str = "/api/echo/price";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let client = Client::new();
    let result = client.get(Url::parse(format!("{}{}", DEV_URI, PRICE).as_str()).unwrap())
        .send()
        .await?
        .text()
        .await?;
    println!("result = {}", result);
    Ok(())
}

pub mod Model {
    extern crate serde;
    extern crate serde_derive;
    
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Price {
        ask: i64,
        bid: i64,
        mid: i64,
    }
}

