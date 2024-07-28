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

#[derive(Debug, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "totalEquity", deserialize_with = "parse_string_to_f64")]
    pub total_equity: f64,
    #[serde(rename = "accountIMRate", deserialize_with = "parse_string_to_f64")]
    pub account_im_rate: f64,
    #[serde(rename = "totalMarginBalance", deserialize_with = "parse_string_to_f64")]
    pub total_margin_balance: f64,
    #[serde(rename = "totalInitialMargin", deserialize_with = "parse_string_to_f64")]
    pub total_initial_margin: f64,
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "totalAvailableBalance", deserialize_with = "parse_string_to_f64")]
    pub total_available_balance: f64,
    #[serde(rename = "accountMMRate", deserialize_with = "parse_string_to_f64")]
    pub account_mm_rate: f64,
    #[serde(rename = "totalPerpUPL", deserialize_with = "parse_string_to_f64")]
    pub total_perp_upl: f64,
    #[serde(rename = "totalWalletBalance", deserialize_with = "parse_string_to_f64")]
    pub total_wallet_balance: f64,
    #[serde(rename = "accountLTV", deserialize_with = "parse_string_to_f64")]
    pub account_ltv: f64,
    #[serde(rename = "totalMaintenanceMargin", deserialize_with = "parse_string_to_f64")]
    pub total_maintenance_margin: f64,
    pub coin: Vec<CoinInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CoinInfo {
    #[serde(rename = "availableToBorrow")]
    pub available_to_borrow: String,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub bonus: f64,
    #[serde(rename = "accruedInterest", deserialize_with = "parse_string_to_f64")]
    pub accrued_interest: f64,
    #[serde(rename = "availableToWithdraw", deserialize_with = "parse_string_to_f64")]
    pub available_to_withdraw: f64,
    #[serde(rename = "totalOrderIM", deserialize_with = "parse_string_to_f64")]
    pub total_order_im: f64,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub equity: f64,
    #[serde(rename = "totalPositionMM", deserialize_with = "parse_string_to_f64")]
    pub total_position_mm: f64,
    #[serde(rename = "usdValue", deserialize_with = "parse_string_to_f64")]
    pub usd_value: f64,
    #[serde(rename = "unrealisedPnl", deserialize_with = "parse_string_to_f64")]
    pub unrealised_pnl: f64,
    #[serde(rename = "collateralSwitch")]
    pub collateral_switch: bool,
    #[serde(rename = "spotHedgingQty", deserialize_with = "parse_string_to_f64")]
    pub spot_hedging_qty: f64,
    #[serde(rename = "borrowAmount")]
    pub borrow_amount: String,
    #[serde(rename = "totalPositionIM", deserialize_with = "parse_string_to_f64")]
    pub total_position_im: f64,
    #[serde(rename = "walletBalance", deserialize_with = "parse_string_to_f64")]
    pub wallet_balance: f64,
    #[serde(rename = "cumRealisedPnl", deserialize_with = "parse_string_to_f64")]
    pub cum_realised_pnl: f64,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub locked: f64,
    #[serde(rename = "marginCollateral")]
    pub margin_collateral: bool,
    pub coin: String,
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