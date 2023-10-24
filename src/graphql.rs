use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::shopify_payload::admin_product::GraphQLShopifyProductQueryResponse;

#[derive(Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    pub variables: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GraphQLResponseData {
    GraphQLShopifyProductQueryResponse(GraphQLShopifyProductQueryResponse),
}

#[derive(Deserialize, Debug)]
pub struct GraphQLResponse {
    pub data: GraphQLResponseData,
}
