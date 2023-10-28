use std::time::Duration;

use mongodb::bson::Document;
use mongodb::{options::ClientOptions, Client, Collection};
use tokio::time::sleep;
use tracing::{error, info};

use crate::env_config::get_env_config;

const RETRY_COUNT: i32 = 10;
const RETRY_DELAY: Duration = Duration::from_secs(10);

/*
 This function will try to return a redis connection.
 TODO: We should look into caching the response of this function when
 we have successfull connection.
*/
pub async fn redis_conn() {
    todo!();
}

/*
 This function will create a connection to mongodb and return it.
*/
pub async fn mongo_conn() -> Option<Collection<Document>> {
    let mut retries = 0;

    let env_config = get_env_config();

    while retries != RETRY_COUNT {
        if retries > 1 {
            info!("Mongo Connection Attempt: {retries}");
        }

        let client_options = ClientOptions::parse(env_config.mongo_uri.clone()).await;

        match client_options {
            Ok(mut options) => {
                options.app_name = Some("Jobs Queue Process".to_string());
                let client = Client::with_options(options);

                match client {
                    Ok(client) => {
                        let db = client.database(&env_config.mongo_db_name);

                        return Some(db.collection::<Document>("jobs"));
                    }
                    Err(err) => {
                        error!("Failed to get mongo client. Error: {}", err);
                        retries += 1;
                        sleep(RETRY_DELAY).await;
                    }
                }
            }
            Err(err) => {
                error!("Failed to get mongo client. Error: {}", err);
                retries += 1;
                sleep(RETRY_DELAY).await;
            }
        }
    }

    None
}
