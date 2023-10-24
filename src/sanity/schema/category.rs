#![allow(non_snake_case)]

use serde::Serialize;

use crate::sanity::schema::product::SanityProduct;

#[derive(Serialize, Debug)]
pub struct SanityCategoryCalloutItem {
    pub title: String,
}

#[derive(Serialize, Debug)]
pub enum SanityCategoryItems {
    SanityProduct(SanityProduct),
    SanityCategoryCalloutItem(SanityCategoryCalloutItem),
}

#[derive(Serialize, Debug)]
pub struct SanityCategory {
    pub _id: String,
    pub title: String,
    pub shortDescription: String,
    pub items: Vec<SanityCategoryItems>,
}
