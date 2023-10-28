use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PostJobRequestPayload {
    pub message: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponsePayload {
    pub email: String,
    pub message: Vec<String>,
}
