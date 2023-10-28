use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::{options::ClientOptions, Client, Collection};

use crate::env_config::get_env_config;
/*
 This function will try to return a redis connection.
 TODO: We should look into caching the response of this function when
 we have successfull connection.
*/
pub async fn redis_conn() {
    todo!();
}

/*
 This function will create a connection to mongodb and return it.
*/
pub async fn mongo_conn() -> Result<Collection<Document>, Error> {
    let env_config = get_env_config();

    let client_options = ClientOptions::parse(env_config.mongo_uri).await;

    match client_options {
        Ok(mut options) => {
            options.app_name = Some("Jobs Queue Process".to_string());
            let client = Client::with_options(options);

            match client {
                Ok(client) => {
                    let db = client.database(&env_config.mongo_db_name);

                    Ok(db.collection::<Document>("jobs"))
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}
