#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResp {
    pub order_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelResp {
    pub cancelled_order_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelByClientOidResp {
    pub cancelled_order_id: String,
    pub client_oid: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub id: String,
    pub symbol: String,
    pub op_type: String,
    pub r#type: String,
    pub side: String,
    pub price: String,
    pub size: String,
    pub funds: String,
    pub deal_funds: String,
    pub deal_size: String,
    pub fee: String,
    pub fee_currency: String,
    pub stp: String,
    pub stop: String,
    pub stop_triggered: bool,
    pub stop_price: String,
    pub time_in_force: String,
    pub post_only: bool,
    pub hidden: bool,
    pub iceberg: bool,
    pub visible_size: String,
    pub cancel_after: i64,
    pub channel: String,
    pub client_oid: String,
    pub remark: Option<String>,
    pub tags: Option<String>,
    pub is_active: Option<bool>,
    pub cancel_exist: bool,
    pub created_at: i64,
    pub trade_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalOrder {
    symbol: String,
    deal_price: Option<String>,
    deal_value: Option<String>,
    amount: Option<String>,
    fee: String,
    side: String,
    created_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FillsInfo {
    pub symbol: String,
    pub trade_id: String,
    pub order_id: String,
    pub counter_order_id: String,
    pub side: String,
    pub liquidity: String,
    pub force_taker: bool,
    pub price: String,
    pub size: String,
    pub funds: String,
    pub fee: String,
    pub fee_rate: String,
    pub fee_currency: String,
    pub stop: String,
    pub r#type: String,
    pub created_at: i64,
    pub trade_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOrder {
    pub id: String,
    pub symbol: String,
    pub r#type: String,
    pub side: String,
    pub price: String,
    pub size: u64,
    pub value: String,
    pub deal_value: String,
    pub deal_size: u64,
    pub stp: String,
    pub stop: String,
    pub stop_price_type: String,
    pub stop_triggered: bool,
    pub stop_price: Option<String>,
    pub time_in_force: String,
    pub post_only: bool,
    pub hidden: bool,
    pub iceberg: bool,
    pub leverage: String,
    pub force_hold: bool,
    pub close_order: bool,
    pub visible_size: Option<String>,
    pub client_oid: String,
    pub remark: Option<String>,
    pub tags: Option<String>,
    pub is_active: Option<bool>,
    pub cancel_exist: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub end_at: Option<i64>,
    pub order_time: u64,
    pub settle_currency: String,
    pub status: String,
    pub filled_value: String,
    pub filled_size: u64,
    pub reduce_only: bool
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub id: String,
    pub symbol: String,
    pub auto_deposit: bool,
    pub maint_margin_req: f64,
    pub risk_limit: u32,
    pub real_leverage: f32,
    pub cross_mode: bool,
    pub delev_percentage: f32,
    pub opening_timestamp: u64,
    pub current_timestamp: u64,
    pub current_qty: u64,
    pub current_cost: f64,
    pub current_comm: f64,
    pub unrealised_cost: f64,
    pub realised_gross_cost: f64,
    pub realised_cost: f64,
    pub is_open: bool,
    pub mark_price: f64,
    pub mark_value: f64,
    pub pos_cost: f64,
    pub pos_cross: f32,
    pub pos_init: f64,
    pub pos_comm: f64,
    pub pos_loss: f32,
    pub pos_margin: f64,
    pub pos_maint: f64,
    pub maint_margin: f64,
    pub realised_gross_pnl: f32,
    pub realised_pnl: f64,
    pub unrealised_pnl: f64,
    pub unrealised_pnl_pcnt: f64,
    pub unrealised_roe_pcnt: f64,
    pub avg_entry_price: f32,
    pub liquidation_price: f32,
    pub bankrupt_price: f32,
    pub settle_currency: String,
    pub is_inverse: bool,
    pub maintain_margin: f64
}