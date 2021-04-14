use chrono::Utc;
use failure::Fallible;
use hmac::{Hmac, Mac, NewMac};
use log::debug;
use reqwest::{Client, Method, Response};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::from_str;
use sha2::Sha256;
use url::Url;

pub mod request;
mod util;

use request::Request;
use util::{HeaderBuilder, ToUrlQuery};

const API_URL: &str = "https://ftx.com/api";

#[derive(Debug, Clone)]
struct Auth {
    public_key: String,
    private_key: String,
    subaccount: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct FtxClient {
    client: Client,
    auth: Option<Auth>,
}

#[derive(Deserialize, Debug)]
struct ResponseSchema<T> {
    success: bool,
    result: T,
}

impl FtxClient {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_auth(
        public_key: &str,
        private_key: &str,
        subaccount: Option<String>,
    ) -> Fallible<Self> {
        Ok(Self {
            auth: Some(Auth {
                private_key: private_key.into(),
                public_key: public_key.into(),
                subaccount,
            }),
            client: Default::default(),
        })
    }

    pub fn change_subaccount(&mut self, subaccount: Option<String>) -> Fallible<()> {
        self.auth
            .as_mut()
            .map(|auth| auth.subaccount = subaccount)
            .ok_or_else(|| failure::format_err!("missing auth keys"))
    }

    fn attach_auth_headers<B: HeaderBuilder>(
        &self,
        builder: B,
        method: Method,
        api_path: &str,
        body: Option<&str>,
    ) -> Fallible<B> {
        let auth = self
            .auth
            .as_ref()
            .ok_or_else(|| failure::format_err!("missing auth keys"))?;

        let timestamp = Utc::now().timestamp_millis();

        let prehash = format!(
            "{}{}{}{}",
            timestamp,
            method,
            api_path.strip_suffix("?").unwrap_or(&api_path),
            body.unwrap_or("")
        );
        let mut mac = Hmac::<Sha256>::new_varkey(auth.private_key.as_bytes())
            .map_err(|e| failure::format_err!("{}", e))?;
        mac.update(prehash.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        Ok(builder
            .add_header("FTX-KEY", &auth.public_key)
            .add_header("FTX-TS", &timestamp.to_string())
            .add_header("FTX-SIGN", &signature))
    }

    pub async fn request<Q: Request>(&self, request: Q) -> Fallible<Q::Response> {
        let endpoint = request.render_endpoint();
        let url = format!("{}{}", API_URL, &endpoint);

        let (req, req_path, req_body) = match Q::METHOD {
            Method::GET => {
                let url = Url::parse_with_params(&url, request.to_url_query())?;
                debug!("sending GET message, url: {:?}", &url.as_str());
                (
                    self.client.request(Q::METHOD, url.as_str()),
                    url.path().to_owned() + "?" + url.query().unwrap_or(""),
                    None,
                )
            }
            Method::POST | Method::DELETE => {
                let request_body = serde_json::to_string(&request)?;
                debug!(
                    "sending POST message, url: {:?}, body: {:?}",
                    &url, request_body,
                );
                (
                    self.client
                        .request(Q::METHOD, &url)
                        .body(request_body.clone())
                        .header("content-type", "application/json"),
                    endpoint,
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

        self.handle_response(req.send().await?).await
    }

    async fn handle_response<T: DeserializeOwned + std::fmt::Debug>(
        &self,
        resp: Response,
    ) -> Fallible<T> {
        if resp.status().is_success() {
            let resp = resp.text().await?;
            debug!("got message: {}", &resp);
            match from_str::<ResponseSchema<T>>(&resp) {
                Ok(resp) => {
                    if resp.success {
                        Ok(resp.result)
                    } else {
                        Err(failure::format_err!(
                            "Success = false in response: {:?}",
                            resp
                        ))
                    }
                }
                Err(e) => Err(failure::format_err!(
                    "error {} while deserializing {}",
                    e,
                    resp
                )),
            }
        } else {
            let resp_e = resp.error_for_status_ref().unwrap_err();
            Err(failure::format_err!(
                "http error: {}; body: {};",
                resp_e,
                resp.text().await?
            ))
        }
    }
}
