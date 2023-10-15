use axum::{http::StatusCode, Json};
use serde::Serialize;
use tokio::sync::mpsc::Sender;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

pub async fn handle(tx: Sender<String>) -> (StatusCode, Json<Response>) {
    tx.send(String::from("job"))
        .await
        .expect("Expected: Send job down the channel.");

    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}
