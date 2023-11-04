use async_graphql::{Context, Error, Object};
use serde_email::is_valid_email;
use tokio::sync::mpsc::Sender;
use tracing::error;

use crate::db::mongo_entities::DBMessage;

use crate::graphql::helper_structs::GeneralResponse;

pub struct GraphQLMutationRoot;

#[Object]
impl GraphQLMutationRoot {
    async fn new_message(
        &self,
        ctx: &Context<'_>,
        email: String,
        message: String,
    ) -> Result<GeneralResponse, Error> {
        let is_valid_email = is_valid_email(&email);

        if is_valid_email {
            match ctx.data::<Sender<DBMessage>>() {
                Ok(tx) => {
                    // Send to thread to add to queue.
                    match tx.send(DBMessage { email, message }).await {
                        Ok(_) => {
                            // Return an OK response
                            Ok(GeneralResponse {
                                message: Some(String::from("OK")),
                                error: None,
                            })
                        }
                        Err(err) => {
                            // Return an ERROR response
                            error!("Failed to send job down the channel. Error: {}", err);
                            Err(Error {
                                message: String::from("Failed to process request."),
                                source: None,
                                extensions: None,
                            })
                        }
                    }
                }
                Err(err) => {
                    // Return an ERROR response
                    error!("Failed to get hold of transmitter. Error: {}", err.message);
                    Err(Error {
                        message: String::from("Failed to get hold of transmitter"),
                        source: None,
                        extensions: None,
                    })
                }
            }
        } else {
            Err(Error {
                message: String::from("Invalid Email"),
                source: None,
                extensions: None,
            })
        }
    }
}
