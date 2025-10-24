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

# Now copy the resources
COPY ./backend/resources ./resources

# Expose the port
# TODO define this in an env var which is read by this Dockerfile and also by main.rs
EXPOSE 7878

# How the healthcheck works, inside the container
# exec 3<>/dev/tcp/localhost/7878 -- open a TCP connection using bash's built-in support for TCP sockets
# echo -e "GET /health ...        -- send a GET request to the /health endpoint. The result will look like
#
#   HTTP/1.1 200 OK
#   access-control-allow-credentials: true
#   vary: origin
#   vary: access-control-request-method
#   vary: access-control-request-headers
#   connection: close
#   content-length: 0
#
# response=$(cat <&3)             -- save the output of file descriptor 3 to a variable called 'response'
# exec 3>&-                       -- closes the file descriptor
# echo "$response" | grep -q ...  -- looks for the string '200 OK' in the output, returns 1 (fails) if not found
HEALTHCHECK --interval=1s --retries=120 CMD bash -c '\
  exec 3<>/dev/tcp/localhost/7878 && \
  echo -e "GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n" >&3 && \
  response=$(cat <&3); \
  exec 3>&-; \
  echo "$response" | grep -q "200 OK" || exit 1'

ENV SUBWAY_DB_MODE="docker"

# Command to run the application
CMD ["./subway-backend"]