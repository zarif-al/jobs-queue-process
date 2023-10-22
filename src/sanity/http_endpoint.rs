use crate::env_config::get_env_config;

#[derive(PartialEq)]
pub enum ApiMode {
    Mutate,
    Query,
    Assets,
}

pub fn get_sanity_endpoint(api_mode: ApiMode) -> String {
    // load env form .env
    let env_config = get_env_config();

    let base_url = format!(
        "https://{}.api.sanity.io/v2021-06-07",
        env_config.sanity_project_id
    );

    let end_point;

    if api_mode == ApiMode::Query {
        end_point = "data/query";
    } else if api_mode == ApiMode::Mutate {
        end_point = "data/mutate"
    } else {
        end_point = "assets/images"
    }

    format!("{}/{}/{}", base_url, end_point, env_config.sanity_dataset)
}
