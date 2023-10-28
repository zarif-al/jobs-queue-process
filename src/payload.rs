use serde::{Deserialize, Serialize};
use serde_email::Email;
#[derive(Serialize)]
pub struct PostJobResponsePayload {
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostJobRequestPayload {
    pub message: String,
    pub email: Email,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestMessagesPayload {
    pub email: Option<Email>,
}

#[derive(Deserialize, Serialize)]
pub struct MessagesResponsePayload {
    pub email: Option<Email>,
    pub messages: Option<Vec<String>>,
    pub error: Option<String>,
}
