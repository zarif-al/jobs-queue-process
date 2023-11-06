use crate::db::{mongo_conn, mongo_message::DBMessage};
use tracing::error;

/**
 * This function inserts the provided message and email to a db.
 *
 * Each message and email combination is inserted as a single doc.
 *
 * If data is successfully inserted into the db then this function
 * will return a unit type or None.
 */
pub async fn insert(message: String, email: String) -> Option<()> {
    let mongo_conn = mongo_conn::<DBMessage>().await;

    match mongo_conn {
        Some(conn) => {
            let data_insert = conn.insert_one(DBMessage { email, message }, None).await;

            match data_insert {
                Ok(_) => Some(()),
                Err(err) => {
                    error!("DB insert failed, Error message: {}", err);
                    None
                }
            }
        }
        None => {
            error!("DB insert failed, Error message: Failed to acquire mongo_db connection");
            None
        }
    }
}
