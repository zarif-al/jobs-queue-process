use shuttle_secrets::SecretStore;
use std::env;

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
 # Panics
 This function will panic if any env is missing.
*/
pub fn get_env_config() -> EnvConfig {
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

/*
 This function will set env from `Secrets.toml` to the standard environment.
 We need our env to be available in may different functions and passing it from shuttles' store
 all the way to the function seemed like an unhealthy approach. (Similar to prop drilling issue when working
 in react.)
*/
pub fn set_env(secrets_store: SecretStore) {
    let redis_url = secrets_store
        .get("REDIS_URL")
        .expect("REDIS_URL is not defined in .env");
    std::env::set_var("REDIS_URL", redis_url);

    let redis_work_queue = secrets_store
        .get("REDIS_WORK_QUEUE")
        .expect("REDIS_WORK_QUEUE is not set in .env");
    std::env::set_var("REDIS_WORK_QUEUE", redis_work_queue);

    let port = secrets_store
        .get("PORT")
        .expect("PORT is not defined in the .env");
    std::env::set_var("PORT", port);

    let mongo_uri = secrets_store
        .get("MONGO_URI")
        .expect("MONGO_URI not found in .env.");
    std::env::set_var("MONGO_URI", mongo_uri);

    let mongo_db_name = secrets_store
        .get("MONGO_DB_NAME")
        .expect("MONGO_DB_NAME is not defined in the .env");
    std::env::set_var("MONGO_DB_NAME", mongo_db_name);

    let gmail_email = secrets_store
        .get("GMAIL_EMAIL")
        .expect("GMAIL_EMAIL is not defined in .env");
    std::env::set_var("GMAIL_EMAIL", gmail_email);

    let gmail_app_password = secrets_store
        .get("GMAIL_APP_PASSWORD")
        .expect("GMAIL_APP_PASSWORD is not defined in .env");
    std::env::set_var("GMAIL_APP_PASSWORD", gmail_app_password);
}
