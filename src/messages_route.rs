use axum::{http::StatusCode, Json};
use mongodb::bson::doc;

use crate::{
    db_connect,
    req_res_structs::{
        GeneralResponse, MessagesRequestPayload, MessagesResponse, MessagesResponseEnum,
    },
};

pub async fn handle(payload: MessagesRequestPayload) -> (StatusCode, Json<MessagesResponseEnum>) {
    let mongo_conn = db_connect::mongo_conn().await;

    match mongo_conn {
        Some(collection) => {
            let filter = doc! { "email" : payload.email.to_string() };

            let request = collection.find(filter, None).await;

            match request {
                Ok(mut cursor) => {
                    let mut messages: Vec<String> = vec![];

                    while cursor.advance().await.unwrap_or(false) {
                        let data = cursor.current();

                        let message = data.get_str("message").unwrap();

                        messages.push(message.to_string());
                    }

                    if messages.len() > 0 {
                        // Return an messages
                        return (
                            StatusCode::OK,
                            Json(MessagesResponseEnum::MessagesResponse(MessagesResponse {
                                email: payload.email,
                                messages,
                            })),
                        );
                    } else {
                        // Return a general response
                        return (
                            StatusCode::OK,
                            Json(MessagesResponseEnum::GeneralResponse(GeneralResponse {
                                message: Some("No messages found for this email".to_string()),
                                error: None,
                            })),
                        );
                    }
                }
                Err(_) => {
                    // Return an error response
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(MessagesResponseEnum::GeneralResponse(GeneralResponse {
                            error: Some(format!("Failed to query data from mongo.")),
                            message: None,
                        })),
                    );
                }
            }
        }
        None => {
            // Return an error response
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MessagesResponseEnum::GeneralResponse(GeneralResponse {
                    error: Some(format!("Failed to get mongo connection")),
                    message: None,
                })),
            );
        }
    }
}
