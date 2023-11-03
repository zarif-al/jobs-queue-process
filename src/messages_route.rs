use axum::{http::StatusCode, Json};
use mongodb::bson::doc;
use tracing::error;

use crate::{
    db_connect,
    req_res_structs::{
        GeneralResponse, MessagesRequestPayload, MessagesResponse, MessagesResponseEnum,
        PostJobRequestPayload,
    },
};

pub async fn handle(payload: MessagesRequestPayload) -> (StatusCode, Json<MessagesResponseEnum>) {
    let mongo_conn = db_connect::mongo_conn::<PostJobRequestPayload>().await;

    match mongo_conn {
        Some(collection) => {
            let mongo_query_filter = doc! { "email" : payload.email.to_string() };

            // collection.find() returns a cursor that streams the results as it gets iterated
            let mongo_result_cursor = collection.find(mongo_query_filter, None).await;

            match mongo_result_cursor {
                Ok(mut cursor) => {
                    let mut messages: Vec<String> = vec![];

                    loop {
                        let current_cursor = cursor.advance().await;

                        match current_cursor {
                            /*
                               If new results are returned by then attempt to
                               access data.
                            */
                            Ok(doc_exists) => {
                                if doc_exists {
                                    match cursor.deserialize_current() {
                                        Ok(data) => {
                                            messages.push(data.message);
                                            continue;
                                        }
                                        Err(err) => {
                                            error!(
                                                "Failed to get data from cursor. Error Message: {}",
                                                err
                                            );
                                            break;
                                        }
                                    }
                                } else {
                                    // The cursor is closed. There is no more data.
                                    break;
                                }
                            }
                            Err(err) => {
                                error!("Failed to advance cursor. Error Message: {}", err);
                                break;
                            }
                        }
                    }

                    if messages.len() > 0 {
                        // Return all messages
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
