#![allow(non_snake_case)]

use crate::sanity::root::SanityReference;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct SanityImage {
    pub _type: String,
    pub asset: SanityReference,
}

#[derive(Serialize, Debug, Clone)]
pub struct SanityCustomImage {
    pub alt: Option<String>,
    pub imageSrc: SanityImage,
}
