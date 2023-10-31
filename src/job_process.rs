use mongodb::bson::doc;

use crate::db_connect::mongo_conn;
use tracing::error;

/**
 * This function inserts the provided message and email to a db.
 *
 * Each message and email combination is inserted as a single doc.
 *
 * If data is successfully inserted into the db then this function
 * will return a unit type or None.
 *
 */
pub async fn db_insert(message: String, email: String) -> Option<()> {
    let mongo_conn = mongo_conn().await;

    match mongo_conn {
        Some(conn) => {
            let data_insert = conn
                .insert_one(doc! {"email": email, "message": message}, None)
                .await;

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
