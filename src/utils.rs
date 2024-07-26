use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

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