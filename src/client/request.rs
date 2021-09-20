use std::collections::HashMap;

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::model::{self};
use rust_decimal::Decimal;

pub trait Request: Serialize {
    type Response: DeserializeOwned + std::fmt::Debug;

    const METHOD: Method;
    const NEEDS_AUTH: bool;

    fn render_endpoint(&self) -> String;
}

/// Obtain a list of all subaccounts
#[derive(Serialize, Clone, Copy, Debug)]
pub struct Subaccounts;

impl Request for Subaccounts {
    type Response = Vec<model::Subaccount>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/subaccounts".into()
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct CreateSubaccount<'a> {
    pub nickname: &'a str,
}

impl<'a> Request for CreateSubaccount<'a> {
    type Response = model::Subaccount;

    const METHOD: Method = Method::POST;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/subaccounts".into()
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubaccountUpdateName<'a> {
    pub nickname: &'a str,
    pub new_nickname: &'a str,
}

impl<'a> Request for SubaccountUpdateName<'a> {
    type Response = ();

    const METHOD: Method = Method::POST;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/subaccounts/update_name".into()
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct DeleteSubaccount<'a> {
    #[serde(skip)]
    pub nickname: &'a str,
}

impl<'a> Request for DeleteSubaccount<'a> {
    type Response = ();

    const METHOD: Method = Method::DELETE;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/subaccounts".into()
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct SubaccountBalances<'a> {
    #[serde(skip)]
    pub nickname: &'a str,
}

impl<'a> Request for SubaccountBalances<'a> {
    type Response = Vec<model::Balance>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        format!("/subaccounts/{}/balances", self.nickname)
    }
}

/// Tranfser funds between subaccounts
#[derive(Serialize, Clone, Copy, Debug)]
pub struct SubaccountTransfer<'a> {
    pub coin: &'a str,
    pub size: Decimal,
    pub source: &'a str,
    pub destination: &'a str,
}

impl<'a> Request for SubaccountTransfer<'a> {
    type Response = model::SubaccountTransferResult;

    const METHOD: Method = Method::POST;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/subaccounts/transfer".into()
    }
}

/// Obtain a list of all assets listed on the exchange
#[derive(Serialize, Clone, Copy, Debug)]
pub struct Markets;

impl Request for Markets {
    type Response = Vec<model::Market>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = false;

    fn render_endpoint(&self) -> String {
        "/markets".into()
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct Market<'a> {
    #[serde(skip)]
    pub market_name: &'a str,
}

impl<'a> Request for Market<'a> {
    type Response = model::Market;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = false;

    fn render_endpoint(&self) -> String {
        format!("/markets/{}", self.market_name)
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct Orderbook<'a> {
    #[serde(skip)]
    pub market_name: &'a str,
    pub depth: Option<u32>,
}

impl<'a> Request for Orderbook<'a> {
    type Response = model::Orderbook;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = false;

    fn render_endpoint(&self) -> String {
        format!("/markets/{}/orderbook", self.market_name)
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct Trades<'a> {
    #[serde(skip)]
    pub market_name: &'a str,
    pub limit: Option<u32>,
    // unix timestamps of start and end times
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

impl<'a> Request for Trades<'a> {
    type Response = Vec<model::Trade>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = false;

    fn render_endpoint(&self) -> String {
        format!("/markets/{}/trades", self.market_name)
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct HistoricalPrices<'a> {
    #[serde(skip)]
    pub market_name: &'a str,
    pub resolution: model::TimeResolution,
    pub limit: Option<u32>,
    // unix timestamps of start and end times
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

impl<'a> Request for HistoricalPrices<'a> {
    type Response = Vec<model::HistoricalPrice>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = false;

    fn render_endpoint(&self) -> String {
        format!("/markets/{}/candles", self.market_name)
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct AccountInformation;

impl Request for AccountInformation {
    type Response = model::AccountInformation;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/account".into()
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct Coins;

impl Request for Coins {
    type Response = Vec<model::Coin>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/wallet/coins".into()
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct Balances;

impl Request for Balances {
    type Response = Vec<model::Balance>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/wallet/balances".into()
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct AllAccountBalances;

impl Request for AllAccountBalances {
    type Response = HashMap<String, Vec<model::Balance>>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/wallet/all_balances".into()
    }
}

/// See [documentation](https://docs.ftx.com/#get-deposit-address)
/// for method names that differ from token names
#[derive(Serialize, Clone, Debug)]
pub struct DepositAddress<'a> {
    #[serde(skip)]
    pub coin: &'a str,
    pub method: Option<&'a str>,
}

impl<'a> Request for DepositAddress<'a> {
    type Response = model::Address;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        format!("/wallet/deposit_address/{}", self.coin)
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct DepositHistory {
    pub limit: Option<u32>,
    // unix timestamps of start and end times
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

impl Request for DepositHistory {
    type Response = Vec<model::TransactionHistoryEntry>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/wallet/deposits".into()
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct WithdrawalHistory {
    pub limit: Option<u32>,
    // unix timestamps of start and end times
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

impl Request for WithdrawalHistory {
    type Response = Vec<model::TransactionHistoryEntry>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/wallet/withdrawals".into()
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct OpenOrders<'a> {
    pub market: Option<&'a str>,
}

impl<'a> Request for OpenOrders<'a> {
    type Response = Vec<model::Order>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/orders".into()
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct OrderHistory<'a> {
    pub market: Option<&'a str>,
    // unix timestamps of start and end times
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    // <= 100, max 100
    pub limit: Option<u32>,
}

impl<'a> Request for OrderHistory<'a> {
    type Response = Vec<model::Order>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/orders/history".into()
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct OpenTriggerOrders<'a> {
    pub market: Option<&'a str>,
}

impl<'a> Request for OpenTriggerOrders<'a> {
    type Response = Vec<model::TriggerOrder>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/conditional_orders".into()
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct TriggerOrderHistory<'a> {
    pub market: Option<&'a str>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    // <= 100, max 100
    pub limit: Option<u32>,
    pub side: Option<model::OrderSide>,
    pub order_type: Option<model::OrderType>,
    #[serde(rename = "type")]
    pub type_: Option<model::TriggerOrderType>,
}

impl<'a> Request for TriggerOrderHistory<'a> {
    type Response = Vec<model::TriggerOrder>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/conditional_orders/history".into()
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Triggers {
    #[serde(skip)]
    pub trigger_order_id: u64,
}

impl Request for Triggers {
    type Response = Vec<model::Trigger>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        format!("/conditional_orders/{}/triggers", self.trigger_order_id)
    }
}

#[derive(Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PlaceOrderTypeInfo {
    Limit { price: Decimal },
    Market,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrder<'a> {
    pub market: &'a str,
    pub side: model::OrderSide,
    #[serde(flatten)]
    pub type_: PlaceOrderTypeInfo,
    pub size: Decimal,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    pub client_id: Option<&'a str>,
}

impl<'a> Request for PlaceOrder<'a> {
    type Response = model::Order;

    const METHOD: Method = Method::POST;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/orders".into()
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderRequestId<'a> {
    Client(&'a str),
    Order(u64),
}

#[derive(Serialize, Clone, Debug)]
pub struct OrderStatus<'a> {
    #[serde(skip)]
    pub order_request_id: OrderRequestId<'a>,
}

impl<'a> Request for OrderStatus<'a> {
    type Response = model::Order;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        match &self.order_request_id {
            OrderRequestId::Client(s) => format!("/orders/by_client_id/{}", s),
            OrderRequestId::Order(id) => format!("/orders/{}", id),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModifyOrder<'a> {
    #[serde(skip)]
    pub order_request_id: OrderRequestId<'a>,
    pub price: Option<Decimal>,
    pub size: Option<Decimal>,
    pub client_id: Option<&'a str>,
}

impl<'a> Request for ModifyOrder<'a> {
    type Response = model::Order;

    const METHOD: Method = Method::POST;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        match &self.order_request_id {
            OrderRequestId::Client(s) => format!("/orders/by_client_id/{}/modify", s),
            OrderRequestId::Order(id) => format!("/orders/{}/modify", id),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CancelOrder<'a> {
    #[serde(skip)]
    pub order_request_id: OrderRequestId<'a>,
}

impl<'a> Request for CancelOrder<'a> {
    type Response = String;

    const METHOD: Method = Method::DELETE;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        match &self.order_request_id {
            OrderRequestId::Client(s) => format!("/orders/by_client_id/{}", s),
            OrderRequestId::Order(id) => format!("/orders/{}", id),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct CancelTriggerOrder {
    #[serde(skip)]
    pub trigger_order_id: u64,
}

impl Request for CancelTriggerOrder {
    type Response = String;

    const METHOD: Method = Method::DELETE;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        format!("/conditional_orders/{}", self.trigger_order_id)
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrders<'a> {
    pub market: Option<&'a str>,
    #[serde(rename = "conditionalOrdersOnly")]
    pub trigger_orders_only: bool,
    pub limit_orders_only: bool,
}

impl<'a> Request for CancelAllOrders<'a> {
    type Response = String;

    const METHOD: Method = Method::DELETE;
    const NEEDS_AUTH: bool = true;

    fn render_endpoint(&self) -> String {
        "/orders".into()
    }
}
