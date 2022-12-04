use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub struct Api {
    base_url: String,
}

impl Api {
    pub fn from(base_url: &str) -> Self { Api { base_url: base_url.to_string() } }
}

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct Request {
    code: u8,
    data: Vec<CandleStick>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CandleStick {
    timestamp: f32,
    #[serde(deserialize_with = "str_or_f64")]
    open: f64,
    #[serde(deserialize_with = "str_or_f64")]
    close: f64,
    #[serde(deserialize_with = "str_or_f64")]
    high: f64,
    #[serde(deserialize_with = "str_or_f64")]
    low: f64,
    #[serde(deserialize_with = "str_or_f64")]
    volume: f64,
    #[serde(deserialize_with = "str_or_f64")]
    amount: f64,
}

fn str_or_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().unwrap_or_default(),
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))? as f64,
        _ => return Err(de::Error::custom("wrong type"))
    })
}

impl Api {
    pub async fn make_request(&self, symbol: &String, time_interval: &String) -> Result<Request, Box<dyn std::error::Error>> {
        let url = format!("{}/open/api/v2/market/kline?symbol={symbol}&interval={time_interval}", self.base_url);

        let req = reqwest::get(url)
            .await?
            .json::<Request>()
            .await?;

        Ok(req)
    }
}

