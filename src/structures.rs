use serde::Deserialize;
use serde_json::Value;
use crate::utils::parse_string_to_f64;
use crate::{OrderType, TradeDirection};


// {"retCode":0,"retMsg":"OK","result":{"orderId":"xxxx","orderLinkId":""},"retExtInfo":{},"time":1722030653718}

#[derive(Deserialize, Debug)]
pub struct BybitResponse {
    #[serde(rename = "retCode")]
    pub ret_code: u64,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: Value,
    #[serde(rename = "retExtInfo")]
    pub ret_ext_info: Value,
    pub time: u128
}

#[derive(Deserialize, Debug)]
pub struct CreateOrderResponse {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String
}

#[derive(Deserialize, Debug)]
pub struct Order {
    #[serde(rename = "avgPrice")]
    pub avg_price: String,
    #[serde(rename = "blockTradeId")]
    pub block_trade_id: String,
    #[serde(rename = "cancelType")]
    pub cancel_type: String,
    #[serde(rename = "closeOnTrigger")]
    pub close_on_trigger: bool,
    #[serde(rename = "createType")]
    pub create_type: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "cumExecFee", deserialize_with = "parse_string_to_f64")]
    pub cum_exec_fee: f64,
    #[serde(rename = "cumExecQty", deserialize_with = "parse_string_to_f64")]
    pub cum_exec_qty: f64,
    #[serde(rename = "cumExecValue", deserialize_with = "parse_string_to_f64")]
    pub cum_exec_value: f64,
    #[serde(rename = "isLeverage")]
    pub is_leverage: String,
    #[serde(rename = "lastPriceOnCreated")]
    pub last_price_on_created: String,
    #[serde(rename = "leavesQty")]
    pub leaves_qty: String,
    #[serde(rename = "leavesValue")]
    pub leaves_value: String,
    #[serde(rename = "marketUnit")]
    pub market_unit: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "orderIv")]
    pub order_iv: String,
    #[serde(rename = "orderLinkId")]
    pub order_link_id: String,
    #[serde(rename = "orderStatus")]
    pub order_status: String,
    #[serde(rename = "orderType")]
    pub order_type: OrderType,
    #[serde(rename = "placeType")]
    pub place_type: String,
    #[serde(rename = "positionIdx")]
    pub position_idx: i32,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub price: f64,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub qty: f64,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    #[serde(rename = "rejectReason")]
    pub reject_reason: String,
    pub side: TradeDirection,
    #[serde(rename = "slLimitPrice")]
    pub sl_limit_price: String,
    #[serde(rename = "slTriggerBy")]
    pub sl_trigger_by: String,
    #[serde(rename = "smpGroup")]
    pub smp_group: i32,
    #[serde(rename = "smpOrderId")]
    pub smp_order_id: String,
    #[serde(rename = "smpType")]
    pub smp_type: String,
    #[serde(rename = "stopLoss")]
    pub stop_loss: String,
    #[serde(rename = "stopOrderType")]
    pub stop_order_type: String,
    pub symbol: String,
    #[serde(rename = "takeProfit")]
    pub take_profit: String,
    #[serde(rename = "timeInForce")]
    pub time_in_force: String,
    #[serde(rename = "tpLimitPrice")]
    pub tp_limit_price: String,
    #[serde(rename = "tpTriggerBy")]
    pub tp_trigger_by: String,
    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,
    #[serde(rename = "triggerBy")]
    pub trigger_by: String,
    #[serde(rename = "triggerDirection")]
    pub trigger_direction: i32,
    #[serde(rename = "triggerPrice")]
    pub trigger_price: String,
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
}



#[cfg(test)]
mod tests {

    use crate::{BybitResponse, CreateOrderResponse};

    #[test]
    pub fn test_response_decoding() {
        let json = r#"{"retCode":0,"retMsg":"OK","result":{"orderId":"xxxx","orderLinkId":""},"retExtInfo":{},"time":1722030653718}"#;

        let resp: BybitResponse = serde_json::from_str(&json).unwrap();
        dbg!(&resp);

        let order_query: CreateOrderResponse = serde_json::from_value(resp.result).unwrap();
        dbg!(order_query);

        let json2 = r#"{"retCode":10010,"retMsg":"Unmatched IP, please check your API key's bound IP addresses.","result":{},"retExtInfo":{},"time":1722154324869}"#;

        let resp: BybitResponse = serde_json::from_str(&json2).unwrap();
        dbg!(&resp);
    }

}