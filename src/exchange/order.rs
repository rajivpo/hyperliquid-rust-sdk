use crate::{
    errors::Error,
    helpers::{float_to_int_for_hashing, float_to_string_for_hashing},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Limit {
    pub tif: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    pub trigger_px: String,
    pub is_market: bool,
    pub tpsl: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Limit(Limit),
    Trigger(Trigger),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cloid(String);

impl Cloid {
    pub fn new(raw_cloid: &str) -> Result<Self> {
        if !raw_cloid.starts_with("0x") || raw_cloid.len() != 34 {
            return Err(Error::CloidHexString);
        }
        if !raw_cloid[2..].chars().all(|c| c.is_digit(16)) {
            return Err(Error::CloidInvalidCharacters);
        }
        Ok(Cloid(raw_cloid.to_uppercase()))
    }

    pub fn from_int(cloid: u128) -> Self {
        let hex_str = format!("0x{:032x}", cloid);
        Cloid(hex_str)
    }

    pub fn to_raw(&self) -> &str {
        &self.0
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub asset: u32,
    pub is_buy: bool,
    pub reduce_only: bool,
    pub limit_px: String,
    pub sz: String,
    pub order_type: Order,
}

pub struct ClientLimit {
    pub tif: String,
}

pub struct ClientTrigger {
    pub trigger_px: f64,
    pub is_market: bool,
    pub tpsl: String,
}

pub enum ClientOrder {
    Limit(ClientLimit),
    Trigger(ClientTrigger),
}
pub struct ClientOrderRequest {
    pub asset: String,
    pub is_buy: bool,
    pub reduce_only: bool,
    pub limit_px: f64,
    pub sz: f64,
    pub order_type: ClientOrder,
}

impl ClientOrderRequest {
    pub(crate) fn convert(self, coin_to_asset: &HashMap<String, u32>) -> Result<OrderRequest> {
        let order_type = match self.order_type {
            ClientOrder::Limit(limit) => Order::Limit(Limit { tif: limit.tif }),
            ClientOrder::Trigger(trigger) => Order::Trigger(Trigger {
                trigger_px: float_to_string_for_hashing(trigger.trigger_px),
                is_market: trigger.is_market,
                tpsl: trigger.tpsl,
            }),
        };
        let &asset = coin_to_asset.get(&self.asset).ok_or(Error::AssetNotFound)?;

        Ok(OrderRequest {
            asset,
            is_buy: self.is_buy,
            reduce_only: self.reduce_only,
            limit_px: float_to_string_for_hashing(self.limit_px),
            sz: float_to_string_for_hashing(self.sz),
            order_type,
        })
    }
    pub(crate) fn create_hashable_tuple(
        &self,
        coin_to_asset: &HashMap<String, u32>,
    ) -> Result<(u32, bool, u64, u64, bool, u8, u64)> {
        let hashed_order_type = self.order_type.get_type()?;
        let &asset = coin_to_asset.get(&self.asset).ok_or(Error::AssetNotFound)?;
        Ok((
            asset,
            self.is_buy,
            float_to_int_for_hashing(self.limit_px),
            float_to_int_for_hashing(self.sz),
            self.reduce_only,
            hashed_order_type.0,
            hashed_order_type.1,
        ))
    }
}

impl ClientOrder {
    pub(crate) fn get_type(&self) -> Result<(u8, u64)> {
        match self {
            ClientOrder::Limit(limit) => match limit.tif.as_str() {
                "Gtc" => Ok((2, 0)),
                "Alo" => Ok((1, 0)),
                "Ioc" => Ok((3, 0)),
                _ => Err(Error::OrderTypeNotFound),
            },
            ClientOrder::Trigger(trigger) => match (trigger.is_market, trigger.tpsl.as_str()) {
                (true, "tp") => Ok((4, float_to_int_for_hashing(trigger.trigger_px))),
                (false, "tp") => Ok((5, float_to_int_for_hashing(trigger.trigger_px))),
                (true, "sl") => Ok((6, float_to_int_for_hashing(trigger.trigger_px))),
                (false, "sl") => Ok((7, float_to_int_for_hashing(trigger.trigger_px))),
                _ => Err(Error::OrderTypeNotFound),
            },
        }
    }
}
