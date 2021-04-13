use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

use crate::model::{self, Fixed9};

pub trait Request: Serialize {
    type Response: DeserializeOwned + std::fmt::Debug;

    const METHOD: Method;
    const NEEDS_AUTH: bool;
    const API_PATH: &'static str;

    fn render_endpoint(&self) -> String {
        Self::API_PATH.into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dummy;

impl<'de> Deserialize<'de> for Dummy {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Dummy {})
    }
}

/// Obtain a list of all assets listed on the exchange
#[derive(Serialize, Clone, Copy, Debug)]
pub struct Markets;

impl Request for Markets {
    type Response = Vec<model::Market>;

    const METHOD: Method = Method::GET;
    const NEEDS_AUTH: bool = false;
    const API_PATH: &'static str = "/markets";
}
