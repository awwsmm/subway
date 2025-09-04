# subway

## local development without Docker

### basics

When running locally with Cargo, all dependencies are in-memory. This allows for fast iteration without worrying about networking, containers, and so on. This method of building the project does not require an Internet connection.

Execute the following command in a terminal

```shell
cargo run
```

and then visit http://localhost:7878/hello in a browser to see a web page.

Visit http://localhost:7878/does-not-exist to see the 404 page.

Visit http://localhost:7878/swagger-ui to see the API documentation.

Press <kbd>control</kbd> + <kbd>C</kbd> in the terminal to shut down the server.

### examples

Test the database by writing to it and reading from it. Create a `User` with an `id` and a `name` by visiting

http://localhost:7878/user/add/12345/Albert

Retrieve that user by visiting

http://localhost:7878/user/get/12345

Note that, due to the in-memory nature of the database, all records are wiped when the application is shut down. If you want a persistent database, read the section on [local development with Docker](#local-development-with-docker).

## local development with Docker

Note that building the application in this way (with Docker) requires an Internet connection.

Build the Docker container image by executing the following command in a terminal

```shell
docker build -t subway .
```

Then, run the application and its dependencies with

```shell
docker-compose up
```

Visit http://localhost:7878/hello in a browser to see a web page.

Visit http://localhost:7878/does-not-exist to see the 404 page.

Visit http://localhost:7878/swagger-ui to see the API documentation.

Press <kbd>control</kbd> + <kbd>C</kbd> in the terminal to shut down the container stack.

### examples

Test the database by writing to it and reading from it. Create a `User` with an `id` and a `name` by visiting

http://localhost:7878/user/add/12345/Albert

Retrieve that user by visiting

http://localhost:7878/user/get/12345

Note that records are persisted on your local disk when the application is shut down. If you want to clear the database, run

```shell
docker-compose down -v
```