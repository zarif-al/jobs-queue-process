use redis::aio::Connection;
use std::time::Duration;
use tokio::time::sleep;

use crate::env_config::get_env_config;

/*
 This function will try to return a redis connection.
 TODO: We should look into caching the response of this function when
 we have successfull connection.
*/
pub async fn redis_conn() -> Option<Connection> {
    // get env config
    let env_config = get_env_config();

    const RETRY_COUNT: i32 = 10;
    const RETRY_DELAY: Duration = Duration::from_secs(10);

    let mut retries = 0;

    while retries != RETRY_COUNT {
        if retries > 1 {
            println!("Connection Attempt: {retries}");
        }

        // try to connect to redis client
        let client = &mut redis::Client::open(format!("redis://{}/", env_config.redis_url));

        match client {
            Ok(client) => {
                // try to get an async connection from client
                match client.get_async_connection().await {
                    Ok(conn) => {
                        return Some(conn);
                    }
                    Err(_) => {
                        println!(
                            "App => Failed to get async connection to db. Sleeping for {} seconds.",
                            RETRY_DELAY.as_secs()
                        );
                        sleep(RETRY_DELAY).await;
                        retries += 1;
                    }
                }
            }
            Err(_) => {
                println!(
                    "App => Failed to get redis client. Sleeping for {} seconds.",
                    RETRY_DELAY.as_secs()
                );
                sleep(RETRY_DELAY).await;
                retries += 1;
            }
        }
    }

    return None;
}
