#![allow(non_snake_case)]

use serde::Serialize;

use crate::{
    sanity::{root::image::SanityCustomImage, root::seo::SanitySEO},
    shopify_payload::product_payload::ShopifyProductStatus,
};

#[derive(Serialize, Debug)]
pub struct SanityProduct {
    pub _type: String,
    pub shopify_id: u64,
    pub admin_graphql_id: String,
    pub title: String,
    pub product_type: String,
    pub shopify_created_at: String,
    pub shopify_updated_at: String,
    pub shopify_published_at: String,
    pub seo: SanitySEO,
    pub status: ShopifyProductStatus,
    pub tags: Vec<String>,
    pub images: Vec<SanityCustomImage>,
}
