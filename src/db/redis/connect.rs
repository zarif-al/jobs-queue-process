/*
This module contains code to connect with redis db.
*/
/*
 TODO: We should look into having connection pools.
*/

use redis::aio::Connection;
use tracing::info;

use crate::env_config::get_env_config;

use crate::db::common::{handle_conn_failure, RETRY_LIMIT};

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
