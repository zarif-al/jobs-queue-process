use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PostJobResponsePayload {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostJobRequestPayload {
    pub message: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestMessagesPayload {
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct MessagesResponsePayload {
    pub email: Option<String>,
    pub messages: Option<Vec<String>>,
    pub error: Option<String>,
}
