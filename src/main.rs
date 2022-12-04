use std::env;

use serde::Serialize;

use crate::candlesticks::{Api, Request};

mod candlesticks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let default_symbol = String::from("BTC_USDT");
    let default_interval = String::from("1m");

    // If no currency is specific, then check btc to usd
    let currency = args.get(1).unwrap_or(&default_symbol);
    let interval = args.get(2).unwrap_or(&default_interval);

    let api = Api::from("https://www.mexc.com");

    let req = api.make_request(currency, interval).await.expect("Request failed");
    println!("{:#?}", req);
    
    Ok(())
}
