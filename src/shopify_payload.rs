/*
    This module contains the struct for shopify's payload.
*/
#![allow(non_snake_case)]

pub mod product;

use serde::{Deserialize, Serialize};

use product::Product;

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
    products: Vec<Product>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadProductDelete {
    pub action: Action,
    productIds: Vec<i32>,
}

// TODO: Create a custom deserializer to manange this payload
// #[derive(Deserialize, Serialize, Debug)]
// pub enum RequestPayload {
//     PayloadProductSync(PayloadProductSync),
//     PayloadProductDelete(PayloadProductDelete),
// }

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestPayload {
    pub action: Action,
    pub products: Option<Vec<Product>>,
    pub productIds: Option<Vec<i32>>,
}
