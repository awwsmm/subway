Architectural Decision Record No. ADR-003

# Why Salvo?

This application is a containerized, Rust-based web server with a REST API. It is possible to [write a web server using only the Rust standard library](https://doc.rust-lang.org/book/ch21-01-single-threaded.html), and even to [add multithreading](https://doc.rust-lang.org/book/ch21-02-multithreaded.html). But we will very quickly want to add the ability to parse URI paths and parameters, return responses in a particular format, and add API documentation. "Rolling our own" for all of these would be a lot of work, and a lot of maintenance, and might be error-prone.

Instead, I opted to use a web server framework. There are [lots of options](https://www.reddit.com/r/rust/comments/18ogwtl/which_web_framework_do_you_use_in_rust/) here

- actix-web
- rocket
- axum
- poem
- salvo

...and many others.

My main concern here was that I wanted to use a framework which is well-maintained, i.e. has recent commits, releases, and pull requests on GitHub. Additionally, I wanted [OpenAPI](https://www.openapis.org/) support, so we can easily document endpoints. The REST API is the public interface of the server, so we want it to be well-documented, which is why I prioritized OpenAPI compatibility.

In the Reddit thread linked above, someone mentioned that Poem had out-of-the-box OpenAPI compatibility, so I began researching which frameworks integrated nicely with OpenAPI. I found [another thread](https://www.reddit.com/r/rust/comments/xannze/web_frameworks_with_integrated_open_api/) which mentioned salvo.

I actually implemented the app using [utoipa](https://github.com/juhaku/utoipa) at first, then realized that the latest commit was from three months prior, and the last update to the `axtix-web` integration was from eleven months prior. This is not captured in the commit history, but I undid these changes and rewrote the initial app in `salvo`.

It was very easy to get up and running with salvo, and I haven't hit any major roadblocks yet. If and when I do, it might be worth looking into another one of the frameworks listed above.