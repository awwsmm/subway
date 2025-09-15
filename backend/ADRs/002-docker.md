Architectural Decision Record No. ADR-002

# Why Docker?

This application could easily run as a standalone executable on a VM or a server, so why did I opt to containerize it with Docker? Even in a project which only has a main executable application and, say, a database, containerization is useful for several reasons.

Firstly, for **reproducibility**. Containerizing an application means that everyone running the application is using not only the same application dependencies (found in `Cargo.toml`), but also the same operating system dependencies (underlying C libraries, etc.). This prevents a whole class of "it works on my machine" errors, both during application development and while running the application.

Second, for **testability**. When this service is running in production, it will be on a production server. We want to ensure that e.g. only public endpoints can be accessed without authorization, responses appear in the expected format, documentation is correct and up-to-date, and so on. These things can be tested _manually_ just as easily with or without containerization, but containerization often makes automated testing easier. Containerized applications can easily have separate databases, containing different information, which allows testing of specific scenarios. This is more difficult without containerization.

Third, for **deployability**. With containers, we can easily scale down an old version of an application and scale up a new one. Upgrading a service like this is more difficult and error-prone without containerization.

So while containerization may seem like unnecessary complexity at first, it will allow us to develop, test, and deploy more easily as the application grows.