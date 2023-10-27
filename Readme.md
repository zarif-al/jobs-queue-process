# Jobs Queue Process

This will be a Rust API that will receive payloads and process them in a queue system. The full feature list is provided below.

## Technology

- Rust
- redis

### Rust crates

- `redis-work-queue`
- `async-graphql`
- `mongodb`

## Requirements

- We will use build this as a `graphql` server.
- An endpoint for receiving payloads with the following format.

    ```rust
    struct Payload{
    	message: String,
    	email: String
    }
    ```

    Endpoint: `/post-job`

- Once the endpoint receives the job it will add to a `redis` queue.
- A separate thread will lease jobs from the queue and process them.

     If the job is successful we will post it into a mongo db and send an email about the job’s success.

    If the job fails we will send an email about the failure.

    > To mimic a heavy job we will make the thread sleep for `5` seconds.
    >

    > Please Note: The package we will use for managing queues. `redis-work-queue` currently does not have a working method of cleaning up jobs. These are jobs that are left in a `processing` state because the server crashed or had been reset mid job progress.
    >
- We will have `1` thread to manage payload post requests and `5` threads to process jobs.
- We will have another endpoint which will accept an email and return all the messages stored under that email.

    Endpoint: `/messages`