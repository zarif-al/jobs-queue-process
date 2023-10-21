#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct ShopifyProductImage {
    id: String,
    altText: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
    src: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShopifyProductOption {
    id: String,
    name: String,
    position: i32,
    values: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShopifyProductPriceRange {
    minVariantPrice: Option<f64>,
    maxVariantPrice: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShopifySelectedOptions {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShopifyProductReference {
    id: String,
    status: ShopifyProductStatus,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShopifyProductVariant {
    id: String,
    title: String,
    compareAtPrice: Option<f64>,
    barcode: Option<String>,
    inventoryPolicy: String,
    inventoryQuantity: i32,
    inventoryManagement: String,
    position: i32,
    requiresShipping: bool,
    sku: String,
    taxable: bool,
    weight: f64,
    weightUnit: String,
    price: String,
    createdAt: String,
    updatedAt: String,
    image: Option<ShopifyProductImage>,
    product: ShopifyProductReference,
    selectedOptions: Vec<ShopifySelectedOptions>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum ShopifyProductStatus {
    Active,
    Archived,
    Draft,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ShopifyProduct {
    pub id: String,
    pub title: String,
    featuredImage: ShopifyProductImage,
    handle: String,
    images: Vec<ShopifyProductImage>,
    options: Vec<ShopifyProductOption>,
    priceRange: ShopifyProductPriceRange,
    productType: String,
    tags: Vec<String>,
    variants: Vec<ShopifyProductVariant>,
    vendor: String,
    status: ShopifyProductStatus,
    publishedAt: String,
    createdAt: String,
    updatedAt: String,
}
