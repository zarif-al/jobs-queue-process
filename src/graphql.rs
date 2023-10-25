use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shopify::admin_product::GraphQLShopifyProductQueryResponse;

#[derive(Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: HashMap<String, String>,
}

// TODO : Think about where to keep this.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GraphQLResponseData {
    GraphQLShopifyProductQueryResponse(GraphQLShopifyProductQueryResponse),
}

// TODO : Think about where to keep this.
#[derive(Deserialize, Debug)]
pub struct GraphQLResponse {
    pub data: GraphQLResponseData,
}
