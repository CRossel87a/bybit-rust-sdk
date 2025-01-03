use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use serde::Deserializer;
use serde_json::Value;
use serde::Deserialize;

pub fn get_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn unlock_keys() -> anyhow::Result<(String, String)>{
    let key: String = env::var("bybit_accesskey")?;
    let secret: String = env::var("bybit_secretkey")?;
    Ok((key, secret))
}

pub fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).floor() / y
}

pub fn parse_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::String(s) => s.parse::<f64>().map_err(serde::de::Error::custom),
        Value::Number(n) => n.as_f64().ok_or_else(|| serde::de::Error::custom("Invalid number")),
        Value::Null => Ok(0.0), // or handle null value as you need
        _ => Err(serde::de::Error::custom("Invalid type")),
    }
}