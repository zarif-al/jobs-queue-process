#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SanityCategoryCalloutItem {
    pub _key: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SanityCategoryItemProductReference {
    pub _key: String,
    pub _ref: String,
    pub _type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SanityCategoryItems {
    SanityProduct(SanityCategoryItemProductReference),
    SanityCategoryCalloutItem(SanityCategoryCalloutItem),
}

/*
 We did not inclue the `shortDescription` field as it is not
 relevant to this app.
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct SanityCategory {
    pub r#_type: String,
    pub _id: String,
    pub title: String,
    pub items: Option<Vec<SanityCategoryItems>>,
}
