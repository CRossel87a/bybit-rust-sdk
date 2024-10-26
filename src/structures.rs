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
    //#[serde(rename = "accruedInterest", deserialize_with = "parse_string_to_f64")]
    #[serde(rename = "accruedInterest")]
    pub accrued_interest: String,
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

#[derive(Debug, Deserialize)]
pub struct ContractInfo {
    pub symbol: String,
    #[serde(rename = "contractType")]
    pub contract_type: String,
    pub status: String,
    #[serde(rename = "baseCoin")]
    pub base_coin: String,
    #[serde(rename = "quoteCoin")]
    pub quote_coin: String,
    #[serde(rename = "launchTime")]
    pub launch_time: String,
    #[serde(rename = "deliveryTime")]
    pub delivery_time: String,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String,
    #[serde(rename = "priceScale")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub price_scale: f64,
    #[serde(rename = "leverageFilter")]
    pub leverage_filter: LeverageFilter,
    #[serde(rename = "priceFilter")]
    pub price_filter: PriceFilter,
    #[serde(rename = "lotSizeFilter")]
    pub lot_size_filter: LotSizeFilter,
    #[serde(rename = "unifiedMarginTrade")]
    pub unified_margin_trade: bool,
    #[serde(rename = "fundingInterval")]
    pub funding_interval: i32,
    #[serde(rename = "settleCoin")]
    pub settle_coin: String,
    #[serde(rename = "copyTrading")]
    pub copy_trading: String,
    #[serde(rename = "upperFundingRate")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub upper_funding_rate: f64,
    #[serde(rename = "lowerFundingRate")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub lower_funding_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct LeverageFilter {
    #[serde(rename = "minLeverage")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub min_leverage: f64,
    #[serde(rename = "maxLeverage")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub max_leverage: f64,
    #[serde(rename = "leverageStep")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub leverage_step: f64,
}

#[derive(Debug, Deserialize)]
pub struct PriceFilter {
    #[serde(rename = "minPrice")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub min_price: f64,
    #[serde(rename = "maxPrice")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub max_price: f64,
    #[serde(rename = "tickSize")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub tick_size: f64,
}

#[derive(Debug, Deserialize)]
pub struct LotSizeFilter {
    #[serde(rename = "maxOrderQty")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub max_order_qty: f64,
    #[serde(rename = "minOrderQty")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub min_order_qty: f64,
    #[serde(rename = "qtyStep")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub qty_step: f64,
    #[serde(rename = "postOnlyMaxOrderQty")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub post_only_max_order_qty: f64,
    #[serde(rename = "maxMktOrderQty")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub max_mkt_order_qty: f64,
    #[serde(rename = "minNotionalValue")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub min_notional_value: f64,
}

#[derive(Debug, Deserialize)]
pub struct PositionInfo {
    #[serde(rename = "adlRankIndicator")]
    pub adl_rank_indicator: i32,
    #[serde(rename = "autoAddMargin")]
    pub auto_add_margin: i32,
    #[serde(rename = "avgPrice")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub avg_price: f64,
    #[serde(rename = "bustPrice")]
    pub bust_price: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
    #[serde(rename = "cumRealisedPnl")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub cum_realised_pnl: f64,
    #[serde(rename = "curRealisedPnl")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub cur_realised_pnl: f64,
    #[serde(rename = "isReduceOnly")]
    pub is_reduce_only: bool,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub leverage: f64,
    #[serde(rename = "leverageSysUpdatedTime")]
    pub leverage_sys_updated_time: String,
    
    //#[serde(rename = "liqPrice")]
    //#[serde(deserialize_with = "parse_string_to_f64")]
    //pub liq_price: f64,
    
    #[serde(rename = "markPrice")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub mark_price: f64,
    #[serde(rename = "mmrSysUpdatedTime")]
    pub mmr_sys_updated_time: String,
    #[serde(rename = "positionBalance")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub position_balance: f64,
    #[serde(rename = "positionIM")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub position_im: f64,
    #[serde(rename = "positionIdx")]
    pub position_idx: i32,
    #[serde(rename = "positionMM")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub position_mm: f64,
    #[serde(rename = "positionStatus")]
    pub position_status: String,
    #[serde(rename = "positionValue")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub position_value: f64,
    #[serde(rename = "riskId")]
    pub risk_id: i32,
    #[serde(rename = "riskLimitValue")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub risk_limit_value: f64,
    pub seq: i64,
    #[serde(rename = "sessionAvgPrice")]
    pub session_avg_price: String,
    pub side: String,
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub size: f64,
    #[serde(rename = "stopLoss")]
    pub stop_loss: String,
    pub symbol: String,
    #[serde(rename = "takeProfit")]
    pub take_profit: String,
    #[serde(rename = "tpslMode")]
    pub tpsl_mode: String,
    #[serde(rename = "tradeMode")]
    pub trade_mode: i32,
    #[serde(rename = "trailingStop")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub trailing_stop: f64,
    #[serde(rename = "unrealisedPnl")]
    #[serde(deserialize_with = "parse_string_to_f64")]
    pub unrealised_pnl: f64,
    #[serde(rename = "updatedTime")]
    pub updated_time: String,
}

#[derive(Debug, Deserialize)]
pub struct TickerData {
    #[serde(rename = "ask1Price", deserialize_with = "parse_string_to_f64")]
    pub ask1_price: f64,
    #[serde(rename = "ask1Size", deserialize_with = "parse_string_to_f64")]
    pub ask1_size: f64,
    pub basis: String,
    #[serde(rename = "basisRate")]
    pub basis_rate: String,
    #[serde(rename = "bid1Price", deserialize_with = "parse_string_to_f64")]
    pub bid1_price: f64,
    #[serde(rename = "bid1Size", deserialize_with = "parse_string_to_f64")]
    pub bid1_size: f64,
    #[serde(rename = "curPreListingPhase")]
    pub cur_pre_listing_phase: String,
    #[serde(rename = "deliveryFeeRate")]
    pub delivery_fee_rate: String,
    #[serde(rename = "deliveryTime", deserialize_with = "parse_string_to_f64")]
    pub delivery_time: f64,
    #[serde(rename = "fundingRate", deserialize_with = "parse_string_to_f64")]
    pub funding_rate: f64,
    #[serde(rename = "highPrice24h", deserialize_with = "parse_string_to_f64")]
    pub high_price_24h: f64,
    #[serde(rename = "indexPrice", deserialize_with = "parse_string_to_f64")]
    pub index_price: f64,
    #[serde(rename = "lastPrice", deserialize_with = "parse_string_to_f64")]
    pub last_price: f64,
    #[serde(rename = "lowPrice24h", deserialize_with = "parse_string_to_f64")]
    pub low_price_24h: f64,
    #[serde(rename = "markPrice", deserialize_with = "parse_string_to_f64")]
    pub mark_price: f64,
    #[serde(rename = "nextFundingTime", deserialize_with = "parse_string_to_f64")]
    pub next_funding_time: f64,
    #[serde(rename = "openInterest", deserialize_with = "parse_string_to_f64")]
    pub open_interest: f64,
    #[serde(rename = "openInterestValue", deserialize_with = "parse_string_to_f64")]
    pub open_interest_value: f64,
    #[serde(rename = "preOpenPrice")]
    pub pre_open_price: String,
    #[serde(rename = "preQty")]
    pub pre_qty: String,
    #[serde(rename = "predictedDeliveryPrice")]
    pub predicted_delivery_price: String,
    #[serde(rename = "prevPrice1h", deserialize_with = "parse_string_to_f64")]
    pub prev_price_1h: f64,
    #[serde(rename = "prevPrice24h", deserialize_with = "parse_string_to_f64")]
    pub prev_price_24h: f64,
    #[serde(rename = "price24hPcnt", deserialize_with = "parse_string_to_f64")]
    pub price_24h_pcnt: f64,
    pub symbol: String,
    #[serde(rename = "turnover24h", deserialize_with = "parse_string_to_f64")]
    pub turnover_24h: f64,
    #[serde(rename = "volume24h", deserialize_with = "parse_string_to_f64")]
    pub volume_24h: f64,
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