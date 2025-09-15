# backend

## backend-only development

When running locally with Cargo, all dependencies are in-memory. This allows for fast iteration without worrying about networking, containers, and so on. This method of building the project does not require an Internet connection.

Execute the following command in a terminal

```shell
cargo run
```

and then visit http://localhost:7878/hello in a browser to see a web page.

Visit http://localhost:7878/does-not-exist to see the 404 page.

Visit http://localhost:7878/swagger-ui to see the API documentation.

Press <kbd>control</kbd> + <kbd>C</kbd> in the terminal to shut down the server.

## examples

Test the database by writing to it and reading from it. Create a `Post` with a random `id` and default `title` by executing

```shell
curl -X POST localhost:7878/post/create
```

That will print output like

```
added new Post to table with id: bd130f53-484a-4aed-a268-847cfca662cd
```

Retrieve that Post by executing

```shell
curl localhost:7878/post/get/bd130f53-484a-4aed-a268-847cfca662cd
```

which will give output like

```
{"id":"bd130f53-484a-4aed-a268-847cfca662cd","title":"default title"}
```

Note that, due to the in-memory nature of the database, all records are wiped when the application is shut down. If you want a persistent database, you'll need Docker. Check out the root [README](../README.md) for more information.
