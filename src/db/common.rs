use std::time::Duration;

use tokio::time::sleep;
use tracing::{error, warn};

pub const RETRY_LIMIT: i32 = 10;
pub const RETRY_DELAY: Duration = Duration::from_secs(10);

pub async fn handle_conn_failure(current_retry_count: i32, db_name: String, err: String) {
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
