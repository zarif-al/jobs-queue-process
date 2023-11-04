/*
This module contains code to connect with mongo db.
*/
/*
 TODO: We should look into having connection pools.
*/

use mongodb::{options::ClientOptions, Client, Collection};
use tracing::info;

use crate::env_config::get_env_config;

use crate::db::common::{handle_conn_failure, RETRY_LIMIT};

/*
 This function will try to return a mongodb connection.

 If connection attempts fail it will wait for `RETRY_DELAY` seconds and
 retry for `RETRY_LIMIT` amount of times

 This function expects a generic type that is passed to the
 collection method to return a typed collection
*/
pub async fn mongo_conn<T>() -> Option<Collection<T>> {
    let env_config = get_env_config();

    // Track the retry attempts
    let mut retries = 0;

    while retries != RETRY_LIMIT {
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
                        return Some(db.collection::<T>("jobs"));
                    }
                    Err(err) => {
                        handle_conn_failure(retries, "mongo".to_string(), err.to_string()).await;
                        retries += 1;
                    }
                }
            }
            Err(err) => {
                handle_conn_failure(retries, "mongo".to_string(), err.to_string()).await;
                retries += 1;
            }
        }
    }

    None
}
