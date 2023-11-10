mod db;
mod env_config;
mod graphql;
mod mail;
mod threads;

use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use graphql::{GraphQLMutationRoot, GraphQLQueryRoot};
use redis_work_queue::{KeyPrefix, WorkQueue};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::info;

use db::mongo_message::DBMessage;
use threads::{process_jobs, queue_jobs};

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .title("Zarif: Rust Queue Processing")
            .finish(),
    )
}
#[shuttle_runtime::main(flavor = "multi_thread", worker_threads = 4)]
async fn axum(
    #[shuttle_secrets::Secrets] secrets_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    // transmitters and receivers to pass job to queue thread
    let (tx, rx) = mpsc::channel::<DBMessage>(32);

    // instantiate grapqhl schema
    let schema = Schema::build(GraphQLQueryRoot, GraphQLMutationRoot, EmptySubscription)
        .data(tx)
        .finish();

    env_config::set_env(secrets_store);

    // get env config
    let env_config = env_config::get_env_config();

    // create work queue
    let work_queue = Arc::new(WorkQueue::new(KeyPrefix::from(env_config.redis_work_queue)));

    // thread to listen and add jobs to queue
    tokio::spawn(queue_jobs(
        String::from("Queue Jobs Thread"),
        rx,
        Arc::clone(&work_queue),
    ));

    // thread to process jobs
    tokio::spawn(process_jobs(
        String::from("Process Jobs Thread: 1"),
        Arc::clone(&work_queue),
    ));

    info!("App listening is ready and listening");

    // build our application
    let router = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    Ok(router.into())

    // setup server address
    // let addr = SocketAddr::from(([127, 0, 0, 1], env_config.port));

    // serve it with hyper on designated port
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .expect("App failed to startup!");
}
