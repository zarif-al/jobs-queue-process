# Sanity Custom Sync

This is a Rust implementation of an API server that will queue and execute jobs.

The goal of this application is to create a multithreaded solution to queue jobs sent from `Sanity Connect` in shopify and execute them.

### Benchmark

There is another [project](https://github.com/lemon-hive/sanity-custom-sync) that implements a similar solution using NestJS. We will consider that as the benchmark.

### Current Implementation
> This will be the base branch for Redis implementations.

This implementation uses a Redis database to store and execute jobs.

We will look at the following redis-queue crates:

- [redis_work_queue](https://docs.rs/redis-work-queue/latest/redis_work_queue/)
  > This project is still in-progress. It does not re-start work that has not been complete. Will need keep an eye on this.
  > We can have a work-around by marking a job as complete and pushing it to the work queue again.
  > However, this will not work when the app is restarted.
- [apalis](https://crates.io/crates/apalis)
  > The documentation is old.
- [sidekiq](https://crates.io/crates/sidekiq)
  > The documentation is old.
- [celery](https://crates.io/crates/celery)
  > The documentation is too verbose

When the application starts we have our app running in the main thread listening for requests, and we have a separate thread (`Processor Thread`) that checks the `jobs` queue for jobs.

### **Main Thread**

When a `post` request hits the `/` route. The following things will happen.

- We will create a new `Item` and add it to the `Work Queue`
- We will return a response of `OK`

### **Processor Thread**

This thread will be created when the application starts, it will run in an infinite loop. It will do the following:

- It will loop infinitely.
- It will wait for a job. If one is available it will lease it with a lease expiry time. We will use `1 minute`
- If found it will send that job to a new thread.

    > In this space we have to figure out how to limit the total number of threads our application uses. Lets say we have a capacity of `4` threads, then we have to wait for one of the threads to finish.
    >
- When the thread finishes it will call `Complete`