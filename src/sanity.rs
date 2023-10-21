pub mod product_schema;
pub mod utils;

use serde::{Deserialize, Serialize};

use self::product_schema::SanityProduct;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Mutations {
    CreateOrReplace(SanityProduct),
}

#[derive(Serialize, Debug)]
pub struct SanityMutationPayload {
    pub mutations: Vec<Mutations>,
}

// TODO: Update Sanity Response Handle
#[derive(Deserialize, Debug)]
pub struct SanityErrorResponse {
    pub description: String,
    pub r#type: String,
}

#[derive(Deserialize, Debug)]
pub struct SanityResponse {
    pub error: Option<SanityErrorResponse>,
}
