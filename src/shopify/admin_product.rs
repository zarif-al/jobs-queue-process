use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShopifyAdminProduct {
    pub productCategory: ProductCategory,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductCategory {
    pub productTaxonomyNode: ProductTaxonomyNode,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductTaxonomyNode {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLShopifyProductQueryResponse {
    pub product: ShopifyAdminProduct,
}
