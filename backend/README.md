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

Visit https://localhost:7878/api-doc to see the API documentation.

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

Get an authentication token by sending a dummy user's username and password to the `/login` endpoint

```shell
export TOKEN=$(curl -k -X POST -H "Content-Type: application/json" -d '{"username":"bob","password":"bob"}' https://localhost:7878/login)
```

Available example users include "bob", "clara" (password: "clara"), and "admin" (password: "admin").

Test the database by writing to it and reading from it. Create one or more `Post`s with random `id`s by executing

```shell
curl -k -X POST https://localhost:7878/posts \
  -H "x-token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '[{"title":"title 1","body":"body 1"},{"title":"title 2","body":"body 2"}]'
```

That will print output like

```
added new Post to table with ids: [f417304a-d2a6-4a91-acfe-fbf9c51e6b86, bd58a9d6-5b0b-43cb-b6ca-d9e6bed66570]
```

Retrieve a Post by executing

```shell
curl -k https://localhost:7878/posts/f417304a-d2a6-4a91-acfe-fbf9c51e6b86
```

(`-X GET` is assumed by default with `curl` and can be omitted) which will give output like

```
{"post_id":"f417304a-d2a6-4a91-acfe-fbf9c51e6b86","author_id":"1943fdc4-8c3b-3d3e-b929-05cd04c8ca82","title":"title 1","body":"body 1"}
```

The `post_id` is an auto-generated random ID associated with this new post. The `author_id` is a unique ID associated with the author of the post (in this case, "bob").

You can also list all Posts (up to some limit) with

```shell
curl -k https://localhost:7878/posts\?limit=5
```

(`\?` is required instead of `?` in a shell) which will return the Posts as a JSON list

```
[{"post_id":"f417304a-d2a6-4a91-acfe-fbf9c51e6b86","author_id":"1943fdc4-8c3b-3d3e-b929-05cd04c8ca82","title":"title 1","body":"body 1"},{"post_id":"bd58a9d6-5b0b-43cb-b6ca-d9e6bed66570","author_id":"1943fdc4-8c3b-3d3e-b929-05cd04c8ca82","title":"title 2","body":"body 2"}]
```

Note that, due to the in-memory nature of the database, all records are wiped when the application is shut down. If you want a persistent database, you'll need Docker. Check out the root [README](../README.md) for more information.
