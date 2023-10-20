use std::env;

use dotenv::dotenv;

pub struct EnvConfig {
    pub redis_url: String,
    pub redis_work_queue: String,
    pub port: String,
    pub sanity_project_id: String,
    pub sanity_auth_token: String,
    pub sanity_dataset: String,
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

    let sanity_project_id =
        env::var("SANITY_PROJECT_ID").expect("SANITY_PROJECT_ID is not defined in .env");
    let sanity_auth_token =
        env::var("SANITY_AUTH_TOKEN").expect("SANITY_AUTH_TOKEN not defined in .env");
    let sanity_dataset = env::var("SANITY_DATASET").expect("SANITY_DATASET not defined in .env");

    EnvConfig {
        port,
        redis_url,
        redis_work_queue,
        sanity_auth_token,
        sanity_dataset,
        sanity_project_id,
    }
}
