#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductImage {
    id: String,
    altText: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
    src: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductOption {
    id: String,
    name: String,
    position: i32,
    values: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductPriceRange {
    minVariantPrice: Option<f64>,
    maxVariantPrice: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SelectedOptions {
    name: String,
    value: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ProductReference {
    id: String,
    status: ProductStatus,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductVariant {
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
    image: Option<ProductImage>,
    product: ProductReference,
    selectedOptions: Vec<SelectedOptions>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    Active,
    Archived,
    Draft,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    id: String,
    pub title: String,
    featuredImage: ProductImage,
    handle: String,
    images: Vec<ProductImage>,
    options: Vec<ProductOption>,
    priceRange: ProductPriceRange,
    productType: String,
    tags: Vec<String>,
    variants: Vec<ProductVariant>,
    vendor: String,
    status: ProductStatus,
    publishedAt: String,
    createdAt: String,
    updatedAt: String,
}
