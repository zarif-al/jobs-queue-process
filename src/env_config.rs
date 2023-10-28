use std::env;

use dotenv::dotenv;

pub struct EnvConfig {
    pub redis_url: String,
    pub redis_work_queue: String,
    pub port: u16,
    pub mongo_uri: String,
    pub mongo_db_name: String,
}

/*
 This function will return a struct containing all env configs.
 TODO: Can/should we cache the response of this function?
*/
pub fn get_env_config() -> EnvConfig {
    dotenv().ok();

    // get redis work queue name
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not defined in .env");
    let redis_work_queue =
        env::var("REDIS_WORK_QUEUE").expect("REDIS_WORK_QUEUE is not set in .env");
    let port = env::var("PORT").expect("PORT is not defined in the .env");

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI not found in .env.");
    let mongo_db_name =
        env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not defined in the .env");

    EnvConfig {
        port: port
            .parse::<u16>()
            .expect("PORT should be parseable as a u16."),
        redis_url,
        redis_work_queue,
        mongo_uri,
        mongo_db_name,
    }
}
