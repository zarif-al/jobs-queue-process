use crate::env_config::get_env_config;

#[derive(PartialEq)]
pub enum ApiMode {
    Mutate,
    Query,
}

pub fn get_url(api_mode: ApiMode) -> String {
    // load env form .env
    let env_config = get_env_config();

    let mode;
    if api_mode == ApiMode::Query {
        mode = "query";
    } else {
        mode = "mutate"
    }

    format!(
        "https://{}.api.sanity.io/v2021-06-07/data/{}/{}",
        env_config.sanity_project_id, mode, env_config.sanity_dataset
    )
}
