pub mod utils;

use anyhow::{anyhow, ensure};
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

#[derive(Serialize, PartialEq)]
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

pub enum JsonMethod {
    Post,
    Get
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

    pub async fn send_request(&self, endpoint: &str, timestamp: u128, signature: &str, params: Value, json_method: JsonMethod) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        let mut headers = HeaderMap::new();
        
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(&api_key)?);
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_str(RECV_WINDOW)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let url = format!("{REST_API_URL}{endpoint}");

        let resp = match json_method {
            JsonMethod::Get => {

                let obj = params.as_object().ok_or_else(|| anyhow!("Expected json object"))?;

                let query_string: Vec<String> = obj.iter().map(|(key, value)| {
                    format!("{}={}", key, value.as_str().unwrap_or(""))
                }).collect();

                let request = query_string.join("&");
                println!("request: {request}");
                let signature = self.make_signature(timestamp, &request)?;

                headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature)?);

                let full_url = format!("{}?{}", url, request);
                println!("full url: {full_url}");

                self.client.get(full_url).headers(headers).send().await?
            },
            JsonMethod::Post => {
                headers.insert("X-BAPI-SIGN", HeaderValue::from_str(signature)?);
                self.client.post(url).headers(headers).json(&params).send().await?
            }
        };

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
        let resp = self.send_request(endpoint, timestamp, &signature, params, JsonMethod::Post).await?;
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
        let resp = self.send_request(endpoint, timestamp, &signature, params, JsonMethod::Post).await?;
        let txt = resp.text().await.unwrap();


        // {"retCode":0,"retMsg":"OK","result":{"list":[{"orderId":"xxxxx","orderLinkId":""}],"success":"1"},"retExtInfo":{},"time":1722029752786}
        println!("resp: {txt}");

        Ok(())
    }

    pub async fn create_order(&self, category: Category, symbol: &str, side: TradeDirection, order_type: OrderType, q: f64, price: Option<f64>, time_in_force: Option<TimeInForce>) -> anyhow::Result<()> {

        let endpoint = "/v5/order/create";

        if order_type.eq(&OrderType::Limit) {
            ensure!(price.is_some(), "create_order() missing price");
        }

        let mut params = json!({
            "category": category,
            "symbol": symbol,
            "side": side,
            "qty": q.to_string(),
            "orderType": order_type,
        });

        if let Some(p) = price {
            params["price"] = json!(p.to_string());
        }

        if let Some(tip) = time_in_force {
            params["timeInForce"] = json!(tip);
        }

        let raw_request_body = params.to_string();
        println!("{raw_request_body}");

        let timestamp = get_timestamp();

        let signature = self.make_signature(timestamp, &raw_request_body)?;
        let resp = self.send_request(endpoint, timestamp, &signature, params, JsonMethod::Post).await?;
        let txt = resp.text().await.unwrap();


        // {"retCode":0,"retMsg":"OK","result":{"orderId":"xxxx","orderLinkId":""},"retExtInfo":{},"time":1722030653718}
        println!("resp: {txt}");

        Ok(())
    }

    pub async fn get_orders(&self, category: Category, symbol: &str, order_id_op: Option<OrderId>) -> anyhow::Result<()> {
        let endpoint = "/v5/order/realtime";

        let mut params = json!({
            "category": category,
            "symbol": symbol,
        });

        if let Some(order_id) = order_id_op {
            match order_id {
                OrderId::OrderID(id) => {
                    params["orderId"] = json!(id);
                },
                OrderId::OrderLinkID(id) => {
                    params["orderLinkId"] = json!(id);
                }
            }
        }

        let raw_request_body = params.to_string();
        println!("{raw_request_body}");

        let timestamp = get_timestamp();

        let signature = self.make_signature(timestamp, &raw_request_body)?;
        let resp = self.send_request(endpoint, timestamp, &signature, params, JsonMethod::Get).await?;
        let txt = resp.text().await.unwrap();
        println!("resp: {txt}");

        /*
        {"retCode":0,"retMsg":"OK","result":{"nextPageCursor":"xxxx","category":"linear","list":[{"symbol":"ETHUSDT","orderType":"Market","orderLinkId":"","slLimitPrice":"0","orderId":"xxxx","cancelType":"UNKNOWN","avgPrice":"3277.01","stopOrderType":"","lastPriceOnCreated":"3277.02","orderStatus":"Filled","createType":"CreateByUser","takeProfit":"","cumExecValue":"32.7701","tpslMode":"","smpType":"None","triggerDirection":0,"blockTradeId":"","isLeverage":"","rejectReason":"EC_NoError","price":"3113.17","orderIv":"","createdTime":"1722034770466","tpTriggerBy":"","positionIdx":0,"timeInForce":"IOC","leavesValue":"0","updatedTime":"1722034770468","side":"Sell","smpGroup":0,"triggerPrice":"","tpLimitPrice":"0","cumExecFee":"0.01802356","leavesQty":"0","slTriggerBy":"","closeOnTrigger":false,"placeType":"","cumExecQty":"0.01","reduceOnly":false,"qty":"0.01","stopLoss":"","marketUnit":"","smpOrderId":"","triggerBy":""}]},"retExtInfo":{},"time":1722034786642}
        
         */

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use utils::unlock_keys;
    use serde_json::json;
    use super::*;

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

    #[tokio::test]
    pub async fn test_create_limit_order() {
        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();
        bybit.create_order(Category::Linear, "ETHUSDT", TradeDirection::Buy, OrderType::Limit, 0.1, Some(3000.21), Some(TimeInForce::GTC)).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_create_market_order() {
        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();
        bybit.create_order(Category::Linear, "ETHUSDT", TradeDirection::Sell, OrderType::Market, 0.01, None, None).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_get_orders() {
        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();
        let order_id = String::from("xxx");
        bybit.get_orders(Category::Linear, "ETHUSDT",Some(OrderId::OrderID(order_id))).await.unwrap();
    }
}
