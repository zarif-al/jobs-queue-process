use axum::{http::StatusCode, Json};
use serde::Serialize;
use std::{borrow::BorrowMut, sync::Arc};
use tokio::{sync::Mutex, time::Instant};

#[derive(Serialize)]
pub struct Response {
    message: String,
}

#[derive(PartialEq)]
pub enum JobStatus {
    COMPLETE,
    INCOMPLETE,
}

pub struct Job {
    pub id: usize,
    pub status: JobStatus,
}

pub async fn handle(
    mut jobs: Arc<Mutex<Vec<Job>>>,
    last_req_time: Arc<Mutex<Instant>>,
) -> (StatusCode, Json<Response>) {
    tokio::spawn(async move {
        {
            println!("\nRoute Thread => Waiting for time lock...");
            let mut last_req_time = last_req_time.lock().await;
            println!("Route Thread => Got time lock.\n");
            *last_req_time = Instant::now();
        }

        let jobs_mutex = jobs.borrow_mut();
        let mut jobs_vector = jobs_mutex.lock().await;
        let vector_length = jobs_vector.len();

        let new_job = Job {
            id: vector_length,
            status: JobStatus::INCOMPLETE,
        };

        println!(
            "Route Thread => Pushing new Job, Job ID => {}\n",
            new_job.id
        );
        jobs_vector.push(new_job);
    });

    (
        StatusCode::OK,
        Json(Response {
            message: String::from("OK"),
        }),
    )
}
