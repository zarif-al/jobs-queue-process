use mongodb::bson::doc;
use tracing::error;

use crate::db::{mongo_conn, mongo_message::DBMessage};

/*
This function will accept an EMAIL and fetch all the DBMessage objects
stored in mongo db that contain the provided EMAIL. It will push the messages
in an STRING vector.

It return an Option<Vec<String>>.

If its successfull in getting data from the db it will returns
a Vec<String> else it returns None
*/
pub async fn get_messages(email: &String) -> Option<Vec<String>> {
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

                    return Some(messages);
                }
                Err(err) => {
                    error!(
                        "Failed to get message list for email: {}. Error: {}",
                        &email, err
                    );
                    None
                }
            }
        }
        None => {
            error!(
                "Failed to get message list for {}. Error: Failed to get mongodb connection.",
                &email
            );
            None
        }
    }
}
