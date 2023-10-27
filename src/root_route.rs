use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handle() -> (StatusCode, Json<Response>) {
    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}
