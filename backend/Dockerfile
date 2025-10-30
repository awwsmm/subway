# Copied from https://users.rust-lang.org/t/building-a-docker-image-of-a-hello-world-rust-app/111660

# Use a Rust base image with Cargo installed
FROM rust:1.88.0 AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY ./backend/Cargo.toml ./backend/Cargo.lock ./

# Create an empty src directory to trick Cargo into thinking it's a valid Rust project
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies without the actual source code to cache dependencies separately
RUN cargo build --release

# Now copy the source code
COPY ./backend/src ./src

# ...and the TLS certificate
COPY ./backend/certs ./certs

# ...and the DB migration code
COPY ./backend/migrations ./migrations

# Build the application
RUN cargo build --release

# Start a new stage to create a smaller image without unnecessary build dependencies
FROM debian:12.11-slim

# Set the working directory
WORKDIR /app/backend

# Copy the built binary from the previous stage
COPY --from=builder /usr/src/app/target/release/subway-backend ./

# Now copy the resources and config
COPY ./backend/resources ./resources
COPY ./backend/config.toml ./config.toml

# Command to run the application
CMD ["./subway-backend"]