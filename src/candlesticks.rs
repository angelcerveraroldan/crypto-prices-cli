use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

use crate::utils::seconds_to_date;

pub struct Api {
    pub base_url: String,
    pub symbol: String,
    pub interval: String,
}

impl Api {
    // Specific request
    pub fn from(base_url: &str, symbol: String, interval: String) -> Self {
        Api { base_url: base_url.to_string(), symbol, interval }
    }

    // Default configuration for mexc exchange
    pub fn mexc() -> Self {
        Api {
            base_url: "https://www.mexc.com".to_string(),
            symbol: "BTC_USDT".to_string(),
            interval: "1m".to_string(),
        }
    }

    pub async fn make_request(&self) -> Result<Request, Box<dyn std::error::Error>> {
        let url = format!("{}/open/api/v2/market/kline?symbol={}&interval={}", self.base_url, self.symbol, self.interval);

        let req = reqwest::get(url)
            .await?
            .json::<Request>()
            .await?;

        Ok(req)
    }
}


// The following was designed to parse data from the MEXC API, modifications needed for it to wor with other APIs
#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Request {
    // Error / Success
    pub code: u8,
    // List of candlesticks
    pub data: Vec<CandleStick>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CandleStick {
    pub timestamp: u32,
    #[serde(deserialize_with = "str_or_f64")]
    pub open: f64,
    #[serde(deserialize_with = "str_or_f64")]
    pub close: f64,
    #[serde(deserialize_with = "str_or_f64")]
    pub high: f64,
    #[serde(deserialize_with = "str_or_f64")]
    pub low: f64,
    #[serde(deserialize_with = "str_or_f64")]
    pub volume: f64,
    #[serde(deserialize_with = "str_or_f64")]
    pub amount: f64,
}

// Deserializer
fn str_or_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().unwrap_or_default(),
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))? as f64,
        _ => return Err(de::Error::custom("Wrong type. String or number expected"))
    })
}


