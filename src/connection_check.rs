use std::time::Duration;

use redis::aio::Connection;
use tokio::time::sleep;

pub async fn redis_conn_check(redis_url: &str) -> Option<Connection> {
    // check if we can connect to db.
    const RETRY_COUNT: i32 = 10;
    const RETRY_DELAY: Duration = Duration::from_secs(10);
    let mut retries = 0;

    while retries != RETRY_COUNT {
        println!("Connection Attempt: {retries}");

        let client = &mut redis::Client::open(format!("redis://{redis_url}/"));

        match client {
            Ok(client) => {
                /*
                   try to get a connection
                */
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

        // let work_queue = WorkQueue::new(KeyPrefix::from("sanity_custom_sync_rust"));
    }

    return None;
}
