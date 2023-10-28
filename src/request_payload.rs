use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RequestPayload {
    pub message: String,
    pub email: String,
}
