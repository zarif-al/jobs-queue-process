#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifyCollection {
    pub id: String,
}
