/*
This module contains the structs needed to define `Message` entity.
*/
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DBMessage {
    pub message: String,
    pub email: String,
}
