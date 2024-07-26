pub mod utils;

use anyhow::anyhow;
use serde::Serialize;
use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use sha2::Sha256;

use reqwest::Client;
use reqwest::{StatusCode, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use utils::get_timestamp;


pub const REST_API_URL: &str = "https://api.bybit.com";
// nl https://api.bybit.nl
// hk https://api.byhkbit.com
// tr https://api.bybit-tr.com

pub const RECV_WINDOW: &str = "5000";

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Spot,
    Linear
}

#[derive(Serialize)]
pub enum TradeDirection {
    Buy,
    Sell
}

#[derive(Serialize)]
pub enum OrderType {
    Market,
    Limit
}

// https://bybit-exchange.github.io/docs/v5/enum#timeinforce
#[derive(Serialize)]
pub enum TimeInForce {
    PostOnly,
    GTC,
    IOC,
    FOK
}

pub enum OrderId {
    OrderID(String),
    OrderLinkID(String)
}

pub struct Bybit {
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub client: Client
}

impl Bybit {

    pub fn new(api_key: Option<String>, api_secret: Option<String>, proxy_url: Option<String>) -> anyhow::Result<Self> {

        let client = match proxy_url {
            Some(url) => {
                let proxy = reqwest::Proxy::all(url)?;
                reqwest::Client::builder().proxy(proxy).build()?
            },
            None => reqwest::Client::new()
        };


        Ok(Self {
            api_key,
            api_secret,
            client
        })
    }

    pub fn sign_request(&self, request: String) -> anyhow::Result<String> {
        let secret_key = self.api_secret.as_ref().ok_or_else(|| anyhow!("Missing secret key"))?;
        let mut signed_key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())?;
        signed_key.update(request.as_bytes());
        let signature = hex::encode(signed_key.finalize().into_bytes());
        println!("signature: {signature}");
        Ok(signature)
    }

    pub fn make_signature(&self, timestamp: u128, raw_request_body: &str) -> anyhow::Result<String> {
        let ts = timestamp.to_string();
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;
        let request = format!("{ts}{api_key}{RECV_WINDOW}{raw_request_body}");
        println!("request: {request}");
        self.sign_request(request)
    }

    pub async fn post_signed(&self, endpoint: &str, timestamp: u128, signature: &str, params: Value) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        let mut headers = HeaderMap::new();
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(signature)?);
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(&api_key)?);
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_str(RECV_WINDOW)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let url = format!("{REST_API_URL}{endpoint}");

        let resp = self.client
        .post(url)
        .headers(headers)
        .json(&params)
        .send().await?;
        Ok(resp)
    }

    pub async fn cancel_order(&self, category: Category, symbol: &str, order_id: OrderId) -> anyhow::Result<()> {
        let endpoint = "/v5/order/cancel";

        let mut params = json!({
            "category": category,
            "symbol": symbol,
        });

        match order_id {
            OrderId::OrderID(id) => {
                params["orderId"] = json!(id);
            },
            OrderId::OrderLinkID(id) => {
                params["orderLinkId"] = json!(id);
            }
        }

        let raw_request_body = params.to_string();
        println!("{raw_request_body}");

        let timestamp = get_timestamp();

        let signature = self.make_signature(timestamp, &raw_request_body)?;
        let resp = self.post_signed(endpoint, timestamp, &signature, params).await?;
        let txt = resp.text().await.unwrap();
        println!("resp: {txt}");

        // {"retCode":0,"retMsg":"OK","result":{"orderId":"xxxxxx","orderLinkId":""},"retExtInfo":{},"time":1722029558512}

        Ok(())
    }

    pub async fn cancel_all_orders(&self, category: Category, symbol: &str) -> anyhow::Result<()> {
        let endpoint = "/v5/order/cancel-all";

        let mut params = json!({
            "category": category,
            "symbol": symbol,
        });

        let raw_request_body = params.to_string();
        println!("{raw_request_body}");

        let timestamp = get_timestamp();

        let signature = self.make_signature(timestamp, &raw_request_body)?;
        let resp = self.post_signed(endpoint, timestamp, &signature, params).await?;
        let txt = resp.text().await.unwrap();


        // {"retCode":0,"retMsg":"OK","result":{"list":[{"orderId":"xxxxx","orderLinkId":""}],"success":"1"},"retExtInfo":{},"time":1722029752786}
        println!("resp: {txt}");

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use utils::unlock_keys;
    use serde_json::json;
    use super::*;

    #[tokio::test]
    async fn test_make_data() {

        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();


        let symbol = "ETHUSDT";
        let qty = "0.1";
        let price = "3000.21";
        let endpoint = "/v5/order/create";

        let data = json!({
            "category": Category::Linear,
            "symbol": symbol,
            "side": TradeDirection::Buy,
            "qty": qty,
            "orderType": OrderType::Limit,
            "price": price,
            "timeInForce": TimeInForce::PostOnly
        });
        let raw_request_body = data.to_string();
        println!("{raw_request_body}");

        let timestamp = get_timestamp();

        let signature = bybit.make_signature(timestamp, &raw_request_body).unwrap();
        let resp = bybit.post_signed(endpoint, timestamp, &signature, data).await.unwrap();
        let txt = resp.text().await.unwrap();
        println!("resp: {txt}");
        // {"retCode":0,"retMsg":"OK","result":{"orderId":"xxxx","orderLinkId":""},"retExtInfo":{},"time":1722029716378}
    }

    #[tokio::test]
    pub async fn test_cancel_order() {
        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();
        let order_id = String::from("xxxxx");
        bybit.cancel_order(Category::Linear, "ETHUSDT", OrderId::OrderID(order_id)).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_cancel_all_orders() {
        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();
        bybit.cancel_all_orders(Category::Linear, "ETHUSDT").await.unwrap();
    }
}
