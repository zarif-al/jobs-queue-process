# Jobs Queue Process

Live url: [Zarif: Rust Queue App](https://jobs-queue-project.shuttleapp.rs/)

This is a multi-threaded rust app that I created to explore Rust and its developer experience.

This is a GraphQL API api wit the following queries and mutations:
```gql
# Queries

# Returns the list of messages stored agains this email from Mongo DB
get_messages(email: String)

# Sends an emails with the list of messages stored against this email from Mongo DB
email_messages(email: String)

# Mutations

# Creates a job to save the provided message against the provided email
new_message(email: String, message: String)
```

There are two separate threads running two functions.

The first thread receives messages from the handler for `new_message` query. Then it creates a job and adds it to Redis queue.

The second thread gets jobs from the redis queue and process them by saving them to a Mongo DB.


## Technology

This makes use of the following technologies.

- Rust
- Redis
- GraphQL
- MongoDB
- Gmail

## How to run
- Create a `Secrets.toml` file with the required environment variables defined in `Secrets.example.toml`.
- Run `cargo shuttle run`