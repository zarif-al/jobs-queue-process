use axum::{http::StatusCode, Json};

use crate::payload::{PostJobResponsePayload, RequestMessagesPayload};

pub async fn handle(payload: RequestMessagesPayload) -> (StatusCode, Json<PostJobResponsePayload>) {
    println!("{:?}", payload);

    // Return OK response
    (
        StatusCode::OK,
        Json(PostJobResponsePayload {
            message: String::from("OK"),
        }),
    )
}
