# backend

## backend-only development

When running locally with Cargo or [Bacon](https://dystroy.org/bacon), all dependencies are in-memory. This allows for fast iteration without worrying about networking, containers, and so on. After dependencies are downloaded, this method of building does not require an internet connection.

Execute the following command in a terminal

```shell
cargo run
```

and then visit https://localhost:7878/hello in a browser to see a web page.

> NOTE: you might get a warning in your browser about this page not being secure. This is because, as of right now, this repo uses self-signed TLS certificates. These are viewed as less secure than certificates issued by a third-party certificate authority. You can ignore these warnings while this project is in development.

Visit https://localhost:7878/does-not-exist to see the 404 page.

Visit https://localhost:7878/swagger-ui to see the API documentation.

Press <kbd>control</kbd> + <kbd>C</kbd> in the terminal to shut down the server.

## hot reloading

Use `bacon` instead of cargo for _hot reloading_ -- if you save any changes to the source code, the app will automatically be rebuilt and rerun

```shell
bacon run-long
```

Use hot reloading when
- you are making lots of small changes and want to test endpoints over and over
- you are writing OpenAPI docs and want to see them update in real-time (requires refreshing) in the browser

Do not use hot reloading when
- you want to keep the in-memory database in place (hot reloading will wipe it out)

## examples

Test the database by writing to it and reading from it. Create one or more `Post`s with random `id`s by executing

> TODO: this currently does not work, as this `POST` endpoint pulls post author information from the Keycloak access token. Running this without Keycloak, therefore, fails.

```shell
curl -X POST -k https://localhost:7878/posts \
  --header "Content-Type: application/json" \
  --data '[{"title": "hello, world!"},{"title": "second post"}]'
```

That will print output like

```
added new Post to table with ids: [bd130f53-484a-4aed-a268-847cfca662cd, 1590e22e-825d-42a5-a794-9655df593465]
```

Retrieve a Post by executing

```shell
curl localhost:7878/post/get/bd130f53-484a-4aed-a268-847cfca662cd
```

(`-X GET` is assumed by default with `curl` and can be omitted) which will give output like

```
{"id":"bd130f53-484a-4aed-a268-847cfca662cd","title":"hello, world!"}
```

You can also list all Posts (up to some limit) with

```shell
curl localhost:7878/posts\?limit=5
```

(`\?` is required instead of `?` in a shell) which will return the Posts as a JSON list

```
[{"id":"3c999d9a-aef6-40a2-a276-3ab6bfba1049","title":"default title"},{"id":"2a71a9f9-604a-4629-a982-3605f94edf44","title":"default title"},{"id":"f3f0b8ef-8b34-4ec6-a8dd-b89ff90fc8bc","title":"default title"},{"id":"811d568b-acc1-4727-943c-8ac7e8177182","title":"default title"},{"id":"1f0f411d-195f-41aa-87c2-a8bffcc9cd64","title":"default title"}]
```

Note that, due to the in-memory nature of the database, all records are wiped when the application is shut down. If you want a persistent database, you'll need Docker. Check out the root [README](../README.md) for more information.
