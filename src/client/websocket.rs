use anyhow::{anyhow, Context as AnyhowContext, Result};
use futures::{
    sink::{Sink, SinkExt},
    stream::Stream,
    task::{Context, Poll},
};
use log::debug;
use pin_project::pin_project;
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{http::Request as HttpRequest, protocol::Message as TungsteniteWSMessage},
    MaybeTlsStream, WebSocketStream,
};

use crate::{
    client::FtxClient,
    model::websocket::{LoginArgs, WsInMessage, WsOutMessage},
};

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

const WS_URL: &str = "wss://ftx.com/ws/";

#[pin_project]
pub struct FtxWebsocket {
    #[pin]
    stream: WSStream,
}

impl FtxClient {
    pub async fn websocket(&self) -> Result<FtxWebsocket> {
        let request = HttpRequest::builder()
            .uri(WS_URL)
            .header("user-agent", "ftx-rs");

        let (stream, _) = connect_async(request.body(())?).await?;

        Ok(FtxWebsocket { stream })
    }

    pub async fn send_ws_auth_msg(&self, ws: &mut FtxWebsocket) -> Result<()> {
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| anyhow!("missing auth keys"))?;

        let timestamp = chrono::Utc::now().timestamp_millis();

        let prehash = format!("{}websocket_login", timestamp,);
        let signature = auth.sign(&prehash)?;

        Ok(ws
            .send(WsOutMessage::Login {
                args: LoginArgs {
                    key: &auth.public_key,
                    time: timestamp,
                    sign: &signature,
                    subaccount: auth.subaccount.as_deref(),
                },
            })
            .await?)
    }
}

impl Stream for FtxWebsocket {
    type Item = Result<WsInMessage>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let poll = this.stream.poll_next(cx);
        poll.map(|msg| msg.map(|msg| msg.map_err(anyhow::Error::from).and_then(parse_message)))
    }
}

fn parse_message(msg: TungsteniteWSMessage) -> Result<WsInMessage> {
    let msg = match msg {
        TungsteniteWSMessage::Text(msg) => msg,
        TungsteniteWSMessage::Binary(_) => return Err(anyhow!("unexpected binary contents")),
        TungsteniteWSMessage::Pong(..) => {
            return Err(anyhow!("recieved pong in unexpected format"))
        }
        TungsteniteWSMessage::Ping(..) => {
            return Err(anyhow!("recieved ping in unexpected format"))
        }
        TungsteniteWSMessage::Close(..) => {
            return Ok(WsInMessage::Closed);
        }
    };

    debug!("Incoming websocket message {}", msg);

    serde_json::from_str(&msg).with_context(|| format!("could not deserialize {}", msg))
}

impl<'a> Sink<WsOutMessage<'a>> for FtxWebsocket {
    type Error = anyhow::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.stream.poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(self: Pin<&mut Self>, msg: WsOutMessage<'a>) -> Result<(), Self::Error> {
        let msg = serde_json::to_string(&msg)?;
        debug!("Sending '{}' through websocket", msg);
        let this = self.project();
        Ok(this.stream.start_send(TungsteniteWSMessage::Text(msg))?)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.stream.poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let this = self.project();
        this.stream.poll_close(cx).map_err(|e| e.into())
    }
}
