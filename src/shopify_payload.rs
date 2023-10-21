/*
    This module contains the struct for shopify's payload.
*/
#![allow(non_snake_case)]

pub mod product_payload;

use serde::{Deserialize, Serialize};

use product_payload::ShopifyProduct;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Create,
    Update,
    Sync,
    Delete,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadProductSync {
    pub action: Action,
    pub products: Vec<ShopifyProduct>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadProductDelete {
    pub action: Action,
    productIds: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum RequestPayload {
    PayloadProductSync(PayloadProductSync),
    PayloadProductDelete(PayloadProductDelete),
}
