# subway

## local development

### running locally with Docker

Build the Docker container image by executing the following command in a terminal

```shell
docker build -t subway .
```

Then run the container with

```shell
docker run -d -p 7878:7878 subway
```

This command will emit an id like

```
6496e30285a668b8806f9ba4f7c46cfe75d199529338728850f91b02ab185ca4
```

Visit http://localhost:7878 in a browser to see the web page.

Visit http://localhost:7878/example (or any other route) to see the 404 page.

Execute the following in a terminal to stop the Docker container from running

```shell
docker stop 6496e30285a668b8806f9ba4f7c46cfe75d199529338728850f91b02ab185ca4
```

### running locally with cargo

Execute the following command in a terminal

```shell
cargo run
```

and then visit http://localhost:7878 in a browser to see the web page.

Visit http://localhost:7878/does-not-exist to see the 404 page.

Press <kbd>control</kbd> + <kbd>C</kbd> in the terminal to shut down the server.

## OpenAPI

OpenAPI route documentation is available at the endpoint `/swagger-ui`, e.g. http://localhost:7878/swagger-ui