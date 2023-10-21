use axum::{http::StatusCode, Json};
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handle() -> (StatusCode, Json<Response>) {
    info!("Route: '/', Ready to accept requests!");
    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}
