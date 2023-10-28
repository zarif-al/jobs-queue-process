use axum::{http::StatusCode, Json};
use mongodb::bson::doc;
use serde::Serialize;

use crate::job_process::job_process;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handle() -> (StatusCode, Json<Response>) {
    job_process("Testing".to_string(), "zarif_al96@outlook.com".to_string()).await;

    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}
