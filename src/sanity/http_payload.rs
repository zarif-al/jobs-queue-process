use serde::Serialize;

use crate::sanity::schema::product::SanityProduct;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Mutations {
    CreateOrReplace(SanityProduct),
}

#[derive(Serialize, Debug)]
pub struct SanityMutationPayload {
    pub mutations: Vec<Mutations>,
}
