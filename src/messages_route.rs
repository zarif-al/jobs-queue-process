use axum::{http::StatusCode, Json};
use mongodb::bson::doc;
use tracing::error;

use crate::{
    db_connect,
    payload::{MessagesResponsePayload, RequestMessagesPayload},
};

pub async fn handle(
    payload: RequestMessagesPayload,
) -> (StatusCode, Json<MessagesResponsePayload>) {
    let mongo_conn = db_connect::mongo_conn().await;

    match payload.email {
        Some(email) => {
            match mongo_conn {
                Some(collection) => {
                    let filter = doc! { "email" : email.to_string() };

                    let request = collection.find(filter, None).await;

                    match request {
                        Ok(mut cursor) => {
                            let mut messages: Vec<String> = vec![];

                            while cursor.advance().await.unwrap_or(false) {
                                let data = cursor.current();

                                let message = data.get_str("message").unwrap();

                                messages.push(message.to_string());
                            }

                            // Return an ok response
                            return (
                                StatusCode::OK,
                                Json(MessagesResponsePayload {
                                    email: Some(email),
                                    messages: Some(messages),
                                    error: None,
                                }),
                            );
                        }
                        Err(_) => {
                            error!(
                                "Failed to query data from mongo. For query: {}",
                                email.clone()
                            );

                            // Return an error response
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(MessagesResponsePayload {
                                    error: Some("Failed to run mongo db query".to_string()),
                                    email: None,
                                    messages: None,
                                }),
                            );
                        }
                    }
                }
                None => {
                    // Return an error response
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(MessagesResponsePayload {
                            error: Some("Failed to get mongo db client".to_string()),
                            email: None,
                            messages: None,
                        }),
                    );
                }
            }
        }
        None => {
            error!("Email parameter not found.");
            return (
                StatusCode::BAD_REQUEST,
                Json(MessagesResponsePayload {
                    email: None,
                    messages: None,
                    error: Some("Please provide an email parameter".to_string()),
                }),
            );
        }
    }
}
