use mongodb::bson::doc;

use crate::db_connect::mongo_conn;
use tracing::{error, info};

pub async fn job_process(message: String, email: String) {
    let mongo_conn = mongo_conn().await;

    match mongo_conn {
        Ok(conn) => {
            let data_insert = conn
                .insert_one(doc! {"email": email, "message": message}, None)
                .await;

            match data_insert {
                Ok(_) => {
                    info!("Inserted document successfully.");
                }
                Err(err) => {
                    error!("Failed to insert document. Error: {}", err);
                }
            }
        }
        Err(err) => {
            error!("Failed to get mongo client. Error: {}", err);
        }
    }
}
