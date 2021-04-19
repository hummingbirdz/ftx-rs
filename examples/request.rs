use failure::Fallible;
#[allow(unused_imports)]
use ftx_rs::{model, request, Fixed9, FtxClient};

async fn req<Q: request::Request + std::fmt::Debug>(c: &FtxClient, req: Q) {
    let s = format!("{:#?}", req);
    println!(
        "###\nrequest: {}\nresponse: {:#?}\n",
        s,
        c.request(req).await
    )
}

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("FTX_PRIVATE").unwrap();
    let public_key: String = std::env::var("FTX_PUBLIC").unwrap();

    let c = FtxClient::with_auth(&public_key, &private_key, None)?;

    //req(&c, request::Subaccounts).await;

    //req(&c, request::CreateSubaccount { nickname: "sub2" }).await;

    //req(
    //    &c,
    //    request::SubaccountUpdateName {
    //        nickname: "sub2",
    //        new_nickname: "sub3",
    //    },
    //)
    //.await;

    //req(&c, request::SubaccountBalances { nickname: "owo" }).await;

    //req(&c, request::DeleteSubaccount { nickname: "sub3" }).await;

    //req(
    //    &c,
    //    request::SubaccountTransfer {
    //        coin: "BNB",
    //        size: "0.01".parse().unwrap(),
    //        source: "sub3",
    //        destination: "main",
    //    },
    //)
    //.await;

    // fairly long, dump output in a file
    //req(&c, request::Markets).await;

    //req(
    //    &c,
    //    request::Market {
    //        market_name: "BALHALF/USD",
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::Orderbook {
    //        market_name: "DOGEBEAR/USD",
    //        depth: None,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::Trades {
    //        market_name: "BTC/USD",
    //        limit: Some(5),
    //        start_time: None,
    //        end_time: None,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::HistoricalPrices {
    //        market_name: "BTC/USD",
    //        resolution: model::TimeResolution::T1d,
    //        limit: Some(5),
    //        start_time: None,
    //        end_time: None,
    //    },
    //)
    //.await;

    //req(&c, request::AccountInformation).await;

    //req(&c, request::Coins).await;

    //req(&c, request::Balances).await;

    //req(&c, request::AllAccountBalances).await;

    //req(
    //    &c,
    //    request::DepositAddress {
    //        coin: "BNB",
    //        method: None,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::DepositHistory {
    //        limit: None,
    //        start_time: None,
    //        end_time: None,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::WithdrawalHistory {
    //        limit: None,
    //        start_time: None,
    //        end_time: None,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::OpenOrders {
    //        market: Some("BNB/USD"),
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::OrderHistory {
    //        market: Some("BNB/USD"),
    //        start_time: None,
    //        end_time: None,
    //        limit: Some(50),
    //    },
    //)
    //.await;

    //req(&c, request::OpenTriggerOrders { market: None }).await;

    //req(
    //    &c,
    //    request::TriggerOrderHistory {
    //        market: None,
    //        side: None,
    //        order_type: None,
    //        type_: None,
    //        start_time: None,
    //        end_time: None,
    //        limit: Some(4),
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::Triggers {
    //        trigger_order_id: 35555071,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::PlaceOrder {
    //        market: "BNB/USD",
    //        side: model::OrderSide::Sell,
    //        type_: request::PlaceOrderTypeInfo::Limit {
    //            price: "600.0".parse().unwrap(),
    //        },
    //        size: "0.01".parse().unwrap(),
    //        ioc: false,
    //        post_only: false,
    //        reduce_only: false,
    //        client_id: Some("owo1"),
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::OrderStatus {
    //        order_request_id: request::OrderRequestId::Client("owo"),
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::ModifyOrder {
    //        order_request_id: request::OrderRequestId::Order(41185331713),
    //        price: Some("640".parse().unwrap()),
    //        size: None,
    //        client_id: Some("owo2"),
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::CancelOrder {
    //        order_request_id: request::OrderRequestId::Client("owo2"),
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::CancelAllOrders {
    //        market: None,
    //        limit_orders_only: false,
    //        trigger_orders_only: false,
    //    },
    //)
    //.await;

    //req(
    //    &c,
    //    request::CancelTriggerOrder {
    //        trigger_order_id: 35574973,
    //    },
    //)
    //.await;

    Ok(())
}
