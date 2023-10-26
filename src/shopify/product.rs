#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifyProductImage {
    pub id: String,
    pub altText: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub src: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifyProductOption {
    pub id: String,
    pub name: String,
    pub position: i32,
    pub values: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifyProductPriceRange {
    pub minVariantPrice: Option<f64>,
    pub maxVariantPrice: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifySelectedOptions {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifyProductReference {
    pub id: String,
    pub status: ShopifyProductStatus,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifyProductVariant {
    pub id: String,
    pub title: String,
    pub compareAtPrice: Option<f64>,
    pub barcode: Option<String>,
    pub inventoryPolicy: String,
    pub inventoryQuantity: i32,
    pub inventoryManagement: String,
    pub position: i32,
    pub requiresShipping: bool,
    pub sku: String,
    pub taxable: bool,
    pub weight: f64,
    pub weightUnit: String,
    pub price: String,
    pub createdAt: String,
    pub updatedAt: String,
    pub image: Option<ShopifyProductImage>,
    pub product: ShopifyProductReference,
    pub selectedOptions: Vec<ShopifySelectedOptions>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ShopifyProductStatus {
    Active,
    Archived,
    Draft,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifyProduct {
    pub id: String,
    pub title: String,
    pub featuredImage: Option<ShopifyProductImage>,
    pub handle: String,
    pub images: Vec<ShopifyProductImage>,
    pub options: Vec<ShopifyProductOption>,
    pub priceRange: ShopifyProductPriceRange,
    pub productType: String,
    pub tags: Vec<String>,
    pub variants: Vec<ShopifyProductVariant>,
    pub vendor: String,
    pub status: ShopifyProductStatus,
    pub publishedAt: String,
    pub createdAt: String,
    pub updatedAt: String,
}
