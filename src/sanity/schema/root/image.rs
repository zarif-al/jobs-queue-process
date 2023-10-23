#![allow(non_snake_case)]

use serde::Serialize;

use crate::sanity::base::reference::SanityReference;

#[derive(Serialize, Debug, Clone)]
pub struct SanityImage {
    pub _type: String,
    pub asset: SanityReference,
}

#[derive(Serialize, Debug, Clone)]
pub struct SanityCustomImage {
    pub alt: Option<String>,
    pub imageSrc: SanityImage,
    // This is only needed if the image is being stored
    // in a vector
    pub _key: Option<String>,
}
