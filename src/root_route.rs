use axum::{http::StatusCode, Json};
use serde::Serialize;

use crate::shopify_payload::RequestPayload;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handle(payload: RequestPayload) -> (StatusCode, Json<Response>) {
    println!("{:?}", payload);

    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}
