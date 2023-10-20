use serde::Serialize;

// TODO : Update product schema
#[derive(Serialize, Debug)]
pub struct SanityProduct {
    pub shopify_id: String,
    pub title: String,
    pub _type: String,
}
