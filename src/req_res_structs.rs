use serde::{Deserialize, Serialize};
use serde_email::Email;

#[derive(Serialize, Deserialize)]
pub struct GeneralResponse {
    pub message: Option<String>,
    pub error: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct PostJobRequestPayload {
    pub message: String,
    pub email: Email,
}

#[derive(Deserialize, Serialize)]
pub struct MessagesRequestPayload {
    pub email: Email,
}

#[derive(Deserialize, Serialize)]
pub struct MessagesResponse {
    pub email: Email,
    pub messages: Vec<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum MessagesResponseEnum {
    MessagesResponse(MessagesResponse),
    GeneralResponse(GeneralResponse),
}
