#![allow(non_snake_case)]

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct SanityReference {
    pub _ref: String,
    pub _type: String,
}
