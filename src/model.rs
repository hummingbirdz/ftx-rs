use std::fmt;

use serde::{
    de::{self, IntoDeserializer},
    Deserialize, Deserializer, Serialize,
};

mod fixed9;

pub use fixed9::Fixed9;

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

fn de_f64_str<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    let s: &str = Deserialize::deserialize(deserializer)?;

    s.parse::<f64>().map_err(de::Error::custom)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
enum MarketType {
    #[serde(rename_all = "camelCase")]
    Spot {
        base_currency: String,
        quote_currency: String,
    },
    #[serde(rename_all = "camelCase")]
    Future { underlying: String },
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    name: String,
    #[serde(flatten)]
    type_: MarketType,
    enabled: bool,
    ask: Fixed9,
    bid: Fixed9,
    price: Fixed9,
    last: Fixed9,
    post_only: bool,
    price_increment: Fixed9,
    size_increment: Fixed9,
    restricted: bool,
    min_provide_size: Fixed9,
    high_leverage_fee_exempt: bool,
    change_1h: f64,
    change_24h: f64,
    change_bod: f64,
    quote_volume_24h: Fixed9,
    volume_usd_24h: f64,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subaccount {
    nickname: String,
    deletable: bool,
    editable: bool,
    competition: bool,
}
