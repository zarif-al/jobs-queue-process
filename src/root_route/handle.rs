use axum::Json;
use reqwest::StatusCode;
use tokio::sync::mpsc::Sender;

use crate::shopify::RequestPayload;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    message: String,
}

/*
 This handler will accept the body of a post request and pass it along
 to a thread.
 Then it will respond with an OK status.
 Caution: This handler does not get called if the JSON parse fails.
*/
pub async fn handle(
    tx: Sender<RequestPayload>,
    payload: RequestPayload,
) -> (StatusCode, Json<Response>) {
    // Send to thread to add to queue.
    tx.send(payload)
        .await
        .expect("Failed to send job down the channel");

    // Return OK response
    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}
