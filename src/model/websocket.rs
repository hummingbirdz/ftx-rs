use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::model::{self, PriceQty};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize)]
pub struct LoginArgs<'a> {
    pub key: &'a str,
    pub sign: &'a str,
    pub time: i64,
    pub subaccount: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "channel", rename_all = "snake_case")]
pub enum Channel {
    Orderbook { market: String },
    Trades { market: String },
    Ticker { market: String },
    Markets,
    Fills,
    Orders,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum WsOutMessage<'a> {
    Login {
        args: LoginArgs<'a>,
    },
    Subscribe {
        #[serde(flatten)]
        channel: Channel,
    },
    Unsubscribe {
        #[serde(flatten)]
        channel: Channel,
    },
    Ping,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub name: String,
    #[serde(flatten)]
    pub type_: model::MarketType,
    pub enabled: bool,
    pub price_increment: Decimal,
    pub size_increment: Decimal,
    pub restricted: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Markets {
    pub data: HashMap<String, Market>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ticker {
    pub bid: Option<Decimal>,
    pub ask: Option<Decimal>,
    pub last: Option<Decimal>,
    pub time: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    pub price: Decimal,
    pub size: Decimal,
    // side of the taker
    pub side: model::OrderSide,
    pub liquidation: bool,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Orderbook {
    pub bids: Vec<PriceQty>,
    pub asks: Vec<PriceQty>,
    pub time: f64,
    pub checksum: u64,
}

// TODO: fix this, we have to do this dance because
// not all channels have a market, but the field for market
// is outside the `data` field
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "channel", rename_all = "snake_case")]
pub enum ChannelData {
    Orderbook { market: String, data: Orderbook },
    Trades { market: String, data: Vec<Trade> },
    Ticker { market: String, data: Ticker },
    Markets { data: Markets },
    // TODO
    Fills,
    Orders { data: model::Order },
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsInMessage {
    Subscribed {
        #[serde(flatten)]
        channel: Channel,
    },
    Unsubsribed {
        #[serde(flatten)]
        channel: Channel,
    },
    Pong,
    Error {
        code: u64,
        msg: String,
    },
    Info {
        code: u64,
        msg: String,
    },
    Partial {
        #[serde(flatten)]
        data: ChannelData,
    },
    Update {
        #[serde(flatten)]
        data: ChannelData,
    },
    Closed,
}
