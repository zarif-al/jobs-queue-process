use async_graphql::{Error, Object};
use mongodb::bson::doc;
use serde_email::is_valid_email;
use tracing::error;

use crate::db::{
    mongo_conn,
    mongo_entities::{DBMessage, DBMessageList},
};

pub struct GraphQLQueryRoot;

#[Object]
impl GraphQLQueryRoot {
    async fn get_messages(&self, email: String) -> Result<DBMessageList, Error> {
        let is_valid_email = is_valid_email(&email);

        if is_valid_email {
            let mongo_conn = mongo_conn::<DBMessage>().await;

            match mongo_conn {
                Some(collection) => {
                    let mongo_query_filter = doc! { "email" : &email };

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

                            return Ok(DBMessageList { messages, email });
                        }
                        Err(_) => {
                            // Return an error response
                            Err(Error {
                                message: String::from("Failed to query data from mongo."),
                                source: None,
                                extensions: None,
                            })
                        }
                    }
                }
                None => {
                    // Return an error response
                    Err(Error {
                        message: String::from("Failed to get mongo connection"),
                        source: None,
                        extensions: None,
                    })
                }
            }
        } else {
            Err(Error {
                message: String::from("Invalid email"),
                source: None,
                extensions: None,
            })
        }
    }
}
