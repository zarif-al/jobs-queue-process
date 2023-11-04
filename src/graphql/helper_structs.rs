use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct GeneralResponse {
    pub message: Option<String>,
    pub error: Option<String>,
}
