use failure::Fallible;
use ftx_rs::{model, FtxClient};
use futures::{SinkExt, StreamExt};

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("FTX_PRIVATE").unwrap();
    let public_key: String = std::env::var("FTX_PUBLIC").unwrap();

    let c = FtxClient::with_auth(&public_key, &private_key, None)?;
    let mut ws = c.websocket().await?;

    c.send_ws_auth_msg(&mut ws).await?;

    // ------ Subscription examples -------

    ws.send(model::websocket::WsOutMessage::Subscribe {
        channel: model::websocket::Channel::Orders,
    })
    .await?;

    // Iterate through message queue
    while let Some(msg) = ws.next().await {
        println!("{:#?}", msg);
    }
    Ok(())
}
