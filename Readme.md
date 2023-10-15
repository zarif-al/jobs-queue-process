# Sanity Custom Sync

This is a Rust implementation of an API server that will queue and execute jobs.

The goal of this application is to create a multithreaded solution to queue jobs sent from `Sanity Connect` in shopify and execute them.

### Benchmark

There is another [project](https://github.com/lemon-hive/sanity-custom-sync) that implements a similar solution using NestJS. We will consider that as the benchmark.

### Current Implementation

This implementation uses a local queue to store and execute jobs.

We have the following variables:
- jobs: `VecDeque`
- last_req_time: `Instant`

Both these are `Arc<Mutex>` types.

When the application starts we have our app running in the main thread listening for requests, and we have a separate thread (`Processor Thread`) that checks the `jobs` queue for jobs.

#### Main Thread

When a `post` request hits the `/` route. The following things will happen.

- We will spawn a new thread (`Route Thread`) to handle the req.
- We will send an immediate `OK` response.
- The `Route Thread` will **wait** to get a lock on the `last_req_time` and update it.
- Then it will **wait** to get a lock on `jobs` queue and push the new job to it.

#### Processor Thread

This thread will be created when the application starts, it will run in an infinite loop. It will do the following:

- It will **wait** to get a lock on the `last_req_time`.
- It will check if more than `60 seconds` have passed since the last request.
- If no
  - It will go to sleep for `60 seconds`
- If yes
  - It will **wait** to get a lock on the jobs queue.
  - It will try to find a job that is has the status `INCOMPLETE`.
    - If found
      - It will process the job. (This has a `sleep` call of `5 seconds` to mock real world processing time)
    - If not found
      - The `Processor Thread` will go to sleep for `60 seconds`