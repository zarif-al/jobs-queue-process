#![allow(non_snake_case)]

use serde::Serialize;

use crate::sanity::schema::product::SanityProduct;

use super::schema::category::{SanityCategory, SanityCategoryItems};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SanityCategoryPatchInsert {
    pub after: String,
    pub items: Vec<SanityCategoryItems>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SanityCategoryPatch {
    pub id: String,
    pub setIfMissing: SanityCategory,
    pub insert: SanityCategoryPatchInsert,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Mutations {
    CreateOrReplace(SanityProduct),
    Create(SanityCategory),
    Patch(SanityCategoryPatch),
}

#[derive(Serialize, Debug)]
pub struct SanityMutationPayload {
    pub mutations: Vec<Mutations>,
}
