/*
 TODO: We should look into having connection pools.
*/
use std::time::Duration;

use mongodb::{options::ClientOptions, Client, Collection};
use redis::aio::Connection;
use tokio::time::sleep;
use tracing::{error, info, warn};

use crate::env_config::get_env_config;

const RETRY_LIMIT: i32 = 10;
const RETRY_DELAY: Duration = Duration::from_secs(10);

/*
 This function will try to return an async redis connection.

 If connection attempts fail it will wait for `RETRY_DELAY` seconds and
 retry for `RETRY_LIMIT` amount of times
*/
pub async fn redis_conn() -> Option<Connection> {
    let env_config = get_env_config();

    // Track the retry attempts
    let mut retries = 0;

    while retries != RETRY_LIMIT {
        if retries > 1 {
            info!("Redis Connection Attempt: {retries}");
        }

        let client = &mut redis::Client::open(format!("redis://{}/", env_config.redis_url));

        match client {
            Ok(client) => {
                let async_conn = client.get_async_connection().await;

                match async_conn {
                    Ok(conn) => {
                        return Some(conn);
                    }
                    Err(err) => {
                        handle_conn_failure(retries, "redis".to_string(), err.to_string()).await;
                        retries += 1;
                    }
                }
            }
            Err(err) => {
                handle_conn_failure(retries, "redis".to_string(), err.to_string()).await;
                retries += 1;
            }
        }
    }

    return None;
}

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

async fn handle_conn_failure(current_retry_count: i32, db_name: String, err: String) {
    if current_retry_count + 1 == RETRY_LIMIT {
        error!(
            "App => Failed to get {} db client/connection. Error Message: {}.",
            db_name, err
        );
    } else {
        warn!(
            "App => Failed to get {} db client/connection. Sleeping for {} seconds.",
            db_name,
            RETRY_DELAY.as_secs()
        );
    }

    sleep(RETRY_DELAY).await;
}
