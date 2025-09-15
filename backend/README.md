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

Test the database by writing to it and reading from it. Create a `Post` with a random `id` and default `title` by executing

```shell
curl -X POST $SUBWAY/post/create
```

That will print output like

```
added new Post to table with id: bd130f53-484a-4aed-a268-847cfca662cd
```

Retrieve that Post by executing

```shell
curl $SUBWAY/post/get/bd130f53-484a-4aed-a268-847cfca662cd
```

which will give output like

```
{"id":"bd130f53-484a-4aed-a268-847cfca662cd","title":"default title"}
```

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

Test the database by writing to it and reading from it. Create a `Post` with a random `id` and default `title` by executing

```shell
curl -X POST $SUBWAY/post/create
```

That will print output like

```
added new Post to table with id: bd130f53-484a-4aed-a268-847cfca662cd
```

Retrieve that Post by executing

```shell
curl $SUBWAY/post/get/bd130f53-484a-4aed-a268-847cfca662cd
```

which will give output like

```
{"id":"bd130f53-484a-4aed-a268-847cfca662cd","title":"default title"}
```

Note that records are persisted on your local disk when the application is shut down. If you want to clear the database, run

```shell
docker-compose down -v
```