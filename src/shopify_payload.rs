/*
    This module contains the struct for shopify's payload.
*/
#![allow(non_snake_case)]

pub mod collection;
pub mod product;

use serde::{Deserialize, Serialize};

use collection::ShopifyCollection;
use product::ShopifyProduct;

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
    pub productIds: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadCollectionsSync {
    pub action: Action,
    pub collections: Vec<ShopifyCollection>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadCollectionsDelete {
    pub action: Action,
    pub collectionIds: Vec<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum RequestPayload {
    PayloadProductSync(PayloadProductSync),
    PayloadProductDelete(PayloadProductDelete),
    PayloadCollectionsSync(PayloadCollectionsSync),
    PayloadCollectionsDelete(PayloadCollectionsDelete),
}
