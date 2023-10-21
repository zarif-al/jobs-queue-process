use axum::{http::StatusCode, Json};
use serde::Serialize;
use tracing::info;

use crate::shopify_payload::RequestPayload;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handle(payload: RequestPayload) -> (StatusCode, Json<Response>) {
    info!("Route: '/', Ready to accept requests!");
    println!("{:?}", payload);

    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}
