use chrono::prelude::*;
use serde::{Deserialize, Serialize};

mod fixed9;
pub mod websocket;

pub use fixed9::Fixed9;

pub type PriceQty = (Fixed9, Fixed9);

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum MarketType {
    #[serde(rename_all = "camelCase")]
    Spot {
        base_currency: String,
        quote_currency: String,
    },
    #[serde(rename_all = "camelCase")]
    Future { underlying: String },
}

/// FIXME: when there are no ask/bid/volume stats (when a market launched in the
/// same day we are asking, for example), we can't parse the responce, since some of the
/// field are `null`, (for example `bid` and `price`)
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub name: String,
    #[serde(flatten)]
    pub type_: MarketType,
    pub enabled: bool,
    pub ask: Fixed9,
    pub bid: Fixed9,
    pub price: Fixed9,
    pub last: Fixed9,
    pub post_only: bool,
    pub price_increment: Fixed9,
    pub size_increment: Fixed9,
    pub restricted: bool,
    pub min_provide_size: Fixed9,
    pub high_leverage_fee_exempt: bool,
    pub change_1h: f64,
    pub change_24h: f64,
    pub change_bod: f64,
    pub quote_volume_24h: f64,
    pub volume_usd_24h: f64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    pub nickname: String,
    pub special: bool,
    pub deletable: bool,
    pub editable: bool,
    pub competition: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountTransferResult {
    pub id: u64,
    pub coin: String,
    pub size: Fixed9,
    pub time: DateTime<Utc>,
    pub notes: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    pub asks: Vec<PriceQty>,
    pub bids: Vec<PriceQty>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub liquidation: bool,
    pub side: OrderSide,
    pub size: Fixed9,
    pub price: Fixed9,
    pub time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
pub enum TimeResolution {
    #[serde(rename = "15")]
    T15s,
    #[serde(rename = "60")]
    T1m,
    #[serde(rename = "300")]
    T5m,
    #[serde(rename = "900")]
    T15m,
    #[serde(rename = "3600")]
    T1h,
    #[serde(rename = "14400")]
    T4h,
    #[serde(rename = "86400")]
    T1d,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalPrice {
    pub close: Fixed9,
    pub high: Fixed9,
    pub low: Fixed9,
    pub open: Fixed9,
    pub start_time: DateTime<Utc>,
    pub volume: f64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub coin: String,
    pub free: Fixed9,
    pub total: Fixed9,
    pub usd_value: f64,
    pub spot_borrow: Fixed9,
    pub available_without_borrow: Fixed9,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub username: String,
    pub backstop_provider: bool,
    pub collateral: Fixed9,
    pub free_collateral: Fixed9,
    pub leverage: f64,
    pub initial_margin_requirement: f64,
    pub liquidating: bool,
    pub maintenance_margin_requirement: f64,
    pub maker_fee: f64,
    pub taker_fee: f64,
    pub margin_fraction: Option<f64>,
    pub open_margin_fraction: Option<f64>,
    pub total_account_value: f64,
    pub total_position_size: f64,
    pub position_limit: Option<f64>,
    pub position_limit_used: Option<f64>,
    pub use_ftt_collateral: bool,
    pub charge_interest_on_negative_usd: bool,
    pub spot_margin_enabled: bool,
    pub spot_lending_enabled: bool,
    pub positions: Vec<Position>,
}

// TODO: implement this (futures)
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    pub id: String,
    pub name: String,
    pub fiat: bool,
    pub is_token: bool,
    pub is_etf: bool,
    pub hidden: bool,
    pub can_deposit: bool,
    pub can_withdraw: bool,
    pub can_convert: bool,
    pub collateral: bool,
    pub collateral_weight: f64,
    pub methods: Vec<String>,
    pub credit_to: Option<String>,
    pub bep2_asset: Option<String>,
    pub erc20_contract: Option<String>,
    pub spl_mint: Option<String>,
    pub usd_fungible: bool,
    pub has_tag: bool,
    pub spot_margin: bool,
    pub index_price: f64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address: String,
    pub tag: Option<String>,
    pub method: String,
}

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DepositStatus {
    Confirmed,
    Unconfirmed,
    Cancelled,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum TransactionType {
    #[serde(rename_all = "camelCase")]
    Normal {
        fee: Fixed9,
        status: DepositStatus,
        confirmations: u32,
        sent_time: DateTime<Utc>,
        confirmed_time: DateTime<Utc>,
        txid: Option<String>,
        address: Address,
    },
    Subaccount {},
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistoryEntry {
    pub coin: String,
    pub id: u64,
    pub size: Fixed9,
    pub time: DateTime<Utc>,
    pub notes: Option<String>,
    #[serde(flatten)]
    pub transaction_type: TransactionType,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    New,
    Open,
    Closed,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: u64,
    pub market: String,
    pub created_at: DateTime<Utc>,
    pub type_: OrderType,
    pub side: OrderSide,
    pub price: Fixed9,
    pub size: Fixed9,
    pub filled_size: Fixed9,
    pub remaining_size: Fixed9,
    pub avg_fill_price: Option<Fixed9>,
    pub status: OrderStatus,
    pub future: Option<String>,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    pub client_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOrderType {
    Stop,
    TrailingStop,
    TakeProfit,
}

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TriggerOrderTypeInfo {
    Stop,
    #[serde(rename_all = "camelCase")]
    TrailingStop {
        trail_start: Fixed9,
        trail_value: Fixed9,
    },
    TakeProfit,
}

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case", tag = "orderType")]
pub enum TriggerUnderlyingOrderTypeInfo {
    Market,
    #[serde(rename_all = "camelCase")]
    Limit {
        order_price: Fixed9,
    },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TriggerOrderStatus {
    Open,
    Cancelled,
    Triggered,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TriggerOrder {
    pub id: u64,
    pub market: String,
    pub created_at: DateTime<Utc>,
    #[serde(flatten)]
    pub type_: TriggerOrderTypeInfo,
    #[serde(flatten)]
    pub underlying_order_type: TriggerUnderlyingOrderTypeInfo,
    pub trigger_price: Fixed9,
    pub side: OrderSide,
    pub size: Fixed9,
    pub filled_size: Fixed9,
    pub avg_fill_price: Option<Fixed9>,
    pub status: TriggerOrderStatus,
    // None if hasn't been triggered yet
    pub triggered_at: Option<DateTime<Utc>>,
    pub future: Option<String>,
    pub reduce_only: bool,
    pub client_id: Option<String>,
    pub retry_until_filled: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", untagged)]
pub enum TriggerInfo {
    #[serde(rename_all = "camelCase")]
    Success {
        order_size: Fixed9,
        filled_size: Fixed9,
        order_id: u64,
    },
    Error {
        error: String,
    },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    time: DateTime<Utc>,
    #[serde(flatten)]
    info: TriggerInfo,
}
