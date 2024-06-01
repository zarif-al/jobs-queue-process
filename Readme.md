# Jobs Queue Process

This is a multi-threaded rust app that I created to explore Rust and its developer experience. The main thread runs the api service and listens for requests.

There are two more separate threads running two functions.

The first thread receives messages from the handler for `new_message` (running on the main thread). Then it creates a job and adds it to the Redis queue.

The second thread gets jobs from the redis queue and process them by saving them to a Mongo DB. This function runs in a loop and keeps trying to get jobs from the redis queue.

## Usage Examples

### Mutation

#### Save a new message
```graphql
# You can modify the value for 'email' and 'message' to your own
# email and message
mutation NewMessage{
  newMessage(email: "zarif_al96@outlook.com", message: "Hi Mom!"){
    error,
    message
  }
}
```

### Queries

#### View messages saved against your email
```graphql
# You can modify the value for 'email'
query GetMessages{
  getMessages(email: "zarif_al96@outlook.com"){
    email,
    messages
  }
}
```

#### Have messages saved against your email address emailed to you
```graphql
# You can modify the value for 'email'
query EmailMessage{
  emailMessages(email:"zarif_al96@outlook.com"){
    error,
    message
  }
}
```



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
