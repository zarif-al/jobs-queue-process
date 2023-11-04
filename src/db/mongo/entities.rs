use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DBMessage {
    pub message: String,
    pub email: String,
}

#[derive(SimpleObject)]
pub struct DBMessageList {
    pub email: String,
    pub messages: Vec<String>,
}
