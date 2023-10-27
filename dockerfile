# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the entire current directory into the container
COPY . .


# # Build and run your Rust application
CMD ["cargo", "run"]