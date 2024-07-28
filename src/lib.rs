pub mod utils;
pub mod structures;

use anyhow::bail;
use anyhow::{anyhow, ensure, Context};
use serde::{Serialize, Deserialize};
use hmac::{Hmac, Mac};
use serde_json::{json, Value};
use sha2::Sha256;

use reqwest::Client;
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderValue};
use utils::get_timestamp;
use structures::*;


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

#[derive(Serialize, Deserialize, Debug)]
pub enum TradeDirection {
    Buy,
    Sell
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
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


#[derive(Serialize, Deserialize, Debug)]
pub enum AccountType {
    UNIFIED
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
        //println!("signature: {signature}");
        Ok(signature)
    }

    pub fn make_signature(&self, timestamp: u128, raw_request_body: &str) -> anyhow::Result<String> {
        let ts = timestamp.to_string();
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;
        let request = format!("{ts}{api_key}{RECV_WINDOW}{raw_request_body}");
        //println!("request to sign: {request}");
        self.sign_request(request)
    }

    pub async fn post_request(&self, endpoint: &str, timestamp: u128, signature: &str, params: Value) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        let mut headers = HeaderMap::new();
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(signature)?);
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(&api_key)?);
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_str(RECV_WINDOW)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let url = format!("{REST_API_URL}{endpoint}");
        let resp = self.client.post(url).headers(headers).json(&params).send().await?;
        Ok(resp)
    }

    pub async fn get_request(&self, endpoint: &str, params: Value) -> anyhow::Result<Response> {
        let api_key = self.api_key.as_ref().ok_or_else(|| anyhow!("Missing api key"))?;

        let timestamp = get_timestamp();

        let mut headers = HeaderMap::new();
        
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(&api_key)?);
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_str(RECV_WINDOW)?);
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let url = format!("{REST_API_URL}{endpoint}");

        let obj = params.as_object().ok_or_else(|| anyhow!("Expected json object"))?;

        let query_string: Vec<String> = obj.iter().map(|(key, value)| {
            format!("{}={}", key, value.as_str().unwrap_or(""))
        }).collect();

        let request = query_string.join("&");
        //println!("request: {request}");
        let signature = self.make_signature(timestamp, &request)?;

        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature)?);

        let full_url = format!("{}?{}", url, request);
        //println!("full url: {full_url}");

        let resp = self.client.get(full_url).headers(headers).send().await?;

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
        //println!("{raw_request_body}");

        let timestamp = get_timestamp();

        let signature = self.make_signature(timestamp, &raw_request_body)?;
        let resp = self.post_request(endpoint, timestamp, &signature, params).await?;
        let txt = resp.text().await?;
        //println!("resp: {txt}");

        // {"retCode":0,"retMsg":"OK","result":{"orderId":"xxxxxx","orderLinkId":""},"retExtInfo":{},"time":1722029558512}

        let resp: BybitResponse = serde_json::from_str(&txt)?;

        if resp.ret_code != 0 {
            bail!("bybit err resp: {}", resp.ret_msg);
        }

        Ok(())
    }

    pub async fn cancel_all_orders(&self, category: Category, symbol: &str) -> anyhow::Result<()> {
        let endpoint = "/v5/order/cancel-all";

        let params = json!({
            "category": category,
            "symbol": symbol,
        });

        let raw_request_body = params.to_string();
        //println!("{raw_request_body}");

        let timestamp = get_timestamp();

        let signature = self.make_signature(timestamp, &raw_request_body)?;
        let resp = self.post_request(endpoint, timestamp, &signature, params).await?;
        let txt = resp.text().await?;


        // {"retCode":0,"retMsg":"OK","result":{"list":[{"orderId":"xxxxx","orderLinkId":""}],"success":"1"},"retExtInfo":{},"time":1722029752786}
        //println!("resp: {txt}");

        let resp: BybitResponse = serde_json::from_str(&txt)?;

        if resp.ret_code != 0 {
            bail!("bybit err resp: {}", resp.ret_msg);
        }

        Ok(())
    }

    pub async fn create_order(&self, category: Category, symbol: &str, side: TradeDirection, order_type: OrderType, q: f64, price: Option<f64>, time_in_force: Option<TimeInForce>) -> anyhow::Result<CreateOrderResponse> {

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
        //println!("{raw_request_body}");

        let timestamp = get_timestamp();

        let signature = self.make_signature(timestamp, &raw_request_body)?;
        let resp = self.post_request(endpoint, timestamp, &signature, params).await?;
        let txt = resp.text().await?;


        // {"retCode":0,"retMsg":"OK","result":{"orderId":"xxxx","orderLinkId":""},"retExtInfo":{},"time":1722030653718}
        //println!("resp: {txt}");

        let resp: BybitResponse = serde_json::from_str(&txt)?;

        if resp.ret_code != 0 {
            bail!("bybit err resp: {}", resp.ret_msg);
        }

        let order_query: CreateOrderResponse = serde_json::from_value(resp.result)?;

       Ok(order_query)
    }

    pub async fn get_orders(&self, category: Category, symbol: &str, order_id_op: Option<OrderId>) -> anyhow::Result<Vec<Order>> {
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

        let resp = self.get_request(endpoint, params).await?;
        let txt = resp.text().await?;
        //println!("resp: {txt}");

        let resp: BybitResponse = serde_json::from_str(&txt)?;

        if resp.ret_code != 0 {
            bail!("bybit err resp: {}", resp.ret_msg);
        }

        let list = resp.result.get("list").ok_or_else(|| anyhow!("No list field"))?;
        let order_list = list.as_array().ok_or_else(|| anyhow!("No order list"))?;

        //dbg!(&order_list);

        let mut orders = vec![];

        for order in order_list.iter() {
            let o: Order = serde_json::from_value(order.clone())?;
            orders.push(o);
        }

        Ok(orders)
    }

    pub async fn get_wallet_balance(&self, account_type: AccountType,symbol_op: Option<&str>) -> anyhow::Result<AccountInfo> {

        let endpoint = "/v5/account/wallet-balance";

        let mut params = json!({
            "accountType": account_type,
        });

        if let Some(symbol) = symbol_op {
            params["symbol"] = json!(symbol);
        }

        let resp = self.get_request(endpoint, params).await?;
        let txt = resp.text().await?;
        //println!("resp: {txt}");

        let resp: BybitResponse = serde_json::from_str(&txt)?;

        if resp.ret_code != 0 {
            bail!("bybit err resp: {}", resp.ret_msg);
        }

        //dbg!(&resp.result);

        let account_obj = resp.result
        .get("list")
        .and_then(Value::as_array)
        .and_then(|arr| arr.first())
        .context("Failed to extract account info from response")?;

        let account_info: AccountInfo = serde_json::from_value(account_obj.clone())?;

        Ok(account_info)
    }
}


#[cfg(test)]
mod tests {
    use std::env;

    use utils::unlock_keys;
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

        let proxy_url = env::var("proxy_url").ok();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), proxy_url).unwrap();
        let resp = bybit.create_order(Category::Linear, "ETHUSDT", TradeDirection::Buy, OrderType::Limit, 0.1, Some(3000.21), Some(TimeInForce::GTC)).await.unwrap();
        dbg!(resp);
    }

    #[tokio::test]
    pub async fn test_create_market_order() {
        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();
        bybit.create_order(Category::Linear, "ETHUSDT", TradeDirection::Buy, OrderType::Market, 0.01, None, None).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_get_orders() {
        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();
        // market order d3075c7f-0cae-4bc0-9ace-e9b5d9055326
        // limit order 1f6b52d0-0d38-4c44-a558-0d2619b58061
        let order_id = String::from("d3075c7f-0cae-4bc0-9ace-e9b5d9055326");
        let orders = bybit.get_orders(Category::Linear, "ETHUSDT",Some(OrderId::OrderID(order_id))).await.unwrap();
        dbg!(orders);
    }

    #[tokio::test]
    pub async fn test_get_wallet_balance() {
        let (api_key, api_secret) = unlock_keys().unwrap();

        let bybit = Bybit::new(Some(api_key), Some(api_secret), None).unwrap();

        let balance = bybit.get_wallet_balance(AccountType::UNIFIED, None).await.unwrap();
        dbg!(balance);
    }
}
