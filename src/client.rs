use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use hmac::{Hmac, Mac, NewMac};
use log::debug;
use reqwest::{Client, Method, Response};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::from_str;
use sha2::Sha256;
use url::Url;

pub mod request;
mod util;
mod websocket;

use request::Request;
use util::{HeaderBuilder, ToUrlQuery};

const API_URL: &str = "https://ftx.com/api";

#[derive(Debug, Clone)]
struct Auth {
    public_key: String,
    private_key: String,
    subaccount: Option<String>,
}

impl Auth {
    /// Compute hmacsha256 hash with FTX private key.
    /// This is used to sign messages sent to the server.
    fn sign(&self, prehash: &str) -> Result<String> {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.private_key.as_bytes())
            .with_context(|| format!("failed to use FTX private key as a HMAC-SHA256 key"))?;
        mac.update(prehash.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }
}

#[derive(Debug, Clone, Default)]
pub struct FtxClient {
    client: Client,
    auth: Option<Auth>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ResponseSchema<T> {
    success: bool,
    result: T,
    has_more_data: Option<bool>,
}

impl FtxClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_auth(
        public_key: &str,
        private_key: &str,
        subaccount: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            auth: Some(Auth {
                private_key: private_key.into(),
                public_key: public_key.into(),
                subaccount,
            }),
            client: Default::default(),
        })
    }

    pub fn change_subaccount(&mut self, subaccount: Option<String>) -> Result<()> {
        self.auth
            .as_mut()
            .map(|auth| auth.subaccount = subaccount)
            .ok_or_else(|| anyhow!("no auth data present, can't change subaccount"))
    }

    fn attach_auth_headers<B: HeaderBuilder>(
        &self,
        builder: B,
        method: Method,
        api_path: &str,
        body: Option<&str>,
    ) -> Result<B> {
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| anyhow!("missing auth data"))?;

        let timestamp = Utc::now().timestamp_millis();

        let prehash = format!(
            "{}{}{}{}",
            timestamp,
            method,
            api_path.strip_suffix('?').unwrap_or(api_path),
            body.unwrap_or("")
        );
        let signature = auth.sign(&prehash)?;

        Ok(builder
            .add_header("FTX-KEY", &auth.public_key)
            .add_header("FTX-TS", &timestamp.to_string())
            .add_header("FTX-SIGN", &signature))
    }

    pub async fn request<Q: Request>(&self, request: Q) -> Result<Q::Response> {
        let endpoint = request.render_endpoint();
        let url = format!("{}{}", API_URL, &endpoint);

        let (req, req_path, req_body) = match Q::METHOD {
            Method::GET => {
                let url = Url::parse_with_params(&url, request.to_url_query())?;
                let path = url.path().to_owned();
                let path = match url.query() {
                    Some(q) => path + "?" + q,
                    None => path,
                };
                debug!("sending GET message, url: {}", &url.as_str());
                (self.client.request(Q::METHOD, url.as_str()), path, None)
            }
            Method::POST | Method::DELETE => {
                let url = Url::parse(&url)?;
                let request_body = serde_json::to_string(&request)?;
                debug!(
                    "sending POST message, url: {:?}, body: {:?}",
                    &url, request_body,
                );
                (
                    self.client
                        .request(Q::METHOD, url.as_str())
                        .body(request_body.clone())
                        .header("content-type", "application/json"),
                    url.path().to_owned(),
                    Some(request_body),
                )
            }
            _ => panic!("Request trait specified an unsupported method"),
        };

        let req = req.header("user-agent", "ftx-rs");

        let req = if Q::NEEDS_AUTH {
            self.attach_auth_headers(req, Q::METHOD, &req_path, req_body.as_deref())?
        } else {
            req
        };

        log::debug!("{:?}", req);

        self.handle_response(req.send().await?).await
    }

    async fn handle_response<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        resp: Response,
    ) -> Result<T> {
        if resp.status().is_success() {
            let resp = resp.text().await?;
            println!("got message: {}", &resp);
            debug!("got message: {}", &resp);
            match from_str::<ResponseSchema<T>>(&resp) {
                Ok(resp) => {
                    if resp.success {
                        Ok(resp.result)
                    } else {
                        Err(anyhow!("success = false in response: {:?}", resp))
                    }
                }
                Err(e) => Err(anyhow!("error {} while deserializing {}", e, resp)),
            }
        } else {
            let resp_e = resp.error_for_status_ref().unwrap_err();
            Err(anyhow!(
                "http error: {}; body: {};",
                resp_e,
                resp.text().await?
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};
    use crate::model::SubaccountTransferResult;
    use rust_decimal::Decimal;
    use rust_decimal::prelude::FromStr;
    use crate::request::{ModifyOrder, OrderRequestId};

    #[test]
    fn decimal_deserialisation() {
        let decimal = "1.234599345987983745987345";
        let json = format!(r#"{{
        "id": 1234,
        "coin": "BTC",
        "size": {},
        "time": "2020-09-01T12:00:00.000Z",
        "notes": "some notes"
        }}"#, decimal);

        let result = from_str::<SubaccountTransferResult>(json.as_str())
            .unwrap();
        assert_eq!(Decimal::from_str(decimal).unwrap(), result.size);
    }

    #[test]
    fn decimal_serialization() {
        let decimal = "1.234599345987983745987345";
        let order = ModifyOrder {
            order_request_id: OrderRequestId::Client("client"),
            price: Option::Some(Decimal::from_str(decimal).unwrap()),
            size: Option::None,
            client_id: Option::None,
        };
        let result = to_string::<ModifyOrder>(&order).unwrap();
        assert_eq!(format!(r#"{{"price":{},"size":null,"clientId":null}}"#, decimal), result);
    }
}
