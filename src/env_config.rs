/*
This module will read and return the enviroment
variables from a `.env` file.
*/
use std::env;

use dotenv::dotenv;

pub struct EnvConfig {
    pub redis_url: String,
    pub redis_work_queue: String,
    pub port: u16,
    pub mongo_uri: String,
    pub mongo_db_name: String,
    pub gmail_email: String,
    pub gmail_app_password: String,
}

/*
 This function will return a struct containing all env configs.
 TODO: Can/should we cache the response of this function?
*/
pub fn get_env_config() -> EnvConfig {
    dotenv().ok();

    // Get all necessary env variables
    // Trigger panic()! if any env is missing
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not defined in .env");

    let redis_work_queue =
        env::var("REDIS_WORK_QUEUE").expect("REDIS_WORK_QUEUE is not set in .env");

    let port = env::var("PORT").expect("PORT is not defined in the .env");

    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI not found in .env.");

    let mongo_db_name =
        env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not defined in the .env");

    let gmail_email = env::var("GMAIL_EMAIL").expect("GMAIL_EMAIL is not defined in .env");
    let gmail_app_password =
        env::var("GMAIL_APP_PASSWORD").expect("GMAIL_APP_PASSWORD is not defined in .env");

    EnvConfig {
        port: port
            .parse::<u16>()
            .expect("PORT should be parseable as a u16."),
        redis_url,
        redis_work_queue,
        mongo_uri,
        mongo_db_name,
        gmail_email,
        gmail_app_password,
    }
}
