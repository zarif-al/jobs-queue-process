mod root_handle;
/**
 * TODO : Look into Axum App State
 */
use axum::{routing::post, Router};
use serde::Serialize;
use std::borrow::BorrowMut;
use std::time::Duration;
use std::{sync::Arc, vec};
use tokio::sync::Mutex;
use tokio::time::{sleep, Instant};

#[derive(Serialize)]
pub struct Response {
    message: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // An Arc Mutex Vector to hold our jobs
    let mut jobs: Arc<Mutex<Vec<root_handle::Job>>> = Arc::new(Mutex::new(vec![]));

    let last_req_time: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));

    // build our application with a single route
    let app = Router::new().route(
        "/",
        post({
            let jobs_clone = Arc::clone(&jobs);
            let last_req_time_clone = Arc::clone(&last_req_time);

            move || root_handle::handle(jobs_clone, last_req_time_clone)
        }),
    );

    /*
       Create the thread to keep checking the job vector in a loop.
    */
    let handle = tokio::spawn(async move {
        loop {
            let time;
            {
                println!("Processor Thread => Waiting for time lock...");
                let last_req_time = last_req_time.lock().await;
                println!("Processor Thread => Got time lock.\n");
                time = last_req_time.clone();
            }

            if time.elapsed().as_secs() > 60 {
                println!(
                    "Processing Thread => More than 60 secs between requests. Resuming jobs...\n"
                );
                let jobs_mutex = jobs.borrow_mut();
                let jobs_vector = jobs_mutex.try_lock();

                match jobs_vector {
                    Ok(mut jobs) => {
                        let job = jobs
                            .iter_mut()
                            .find(|el| el.status == root_handle::JobStatus::INCOMPLETE);

                        match job {
                            Some(job) => {
                                println!("Processor Thread => Performing Job: {}", job.id);
                                sleep(Duration::from_secs(5)).await;
                                job.status = root_handle::JobStatus::COMPLETE;
                                println!("Processor Thread => Job Complete: {}\n", job.id);
                                continue;
                            }
                            None => {
                                println!("Processor Thread => No jobs in queue. Going to sleep!\n");
                                sleep(Duration::from_secs(5)).await;
                                continue;
                            }
                        }
                    }
                    Err(_) => {
                        println!("Processor Thread => Failed to acquire lock. Trying again\n");
                        continue;
                    }
                }
            } else {
                println!(
                    "Processor Thread => Less than 60 seconds between requests. Going to sleep!\n"
                );
                sleep(Duration::from_secs(5)).await;
            }
        }
    });

    // run it with hyper on localhost:4000
    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    handle.await.unwrap();
}
