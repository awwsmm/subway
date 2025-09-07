Architectural Decision Record No. ADR-004

# Why OpenAPI?

[OpenAPI](https://www.openapis.org/) (formerly Swagger) is a [Linux Foundation](https://www.linuxfoundation.org/projects) project and the de facto standard for web API description.

OpenAPI integrations in various languages allow for the easy creation of rich HTML documentation for REST routes. Documenting the endpoints exposed by the backend (and keeping that documentation up-to-date) is important for integration with the frontend. Automated documentation doubly so, as manual documentation can easily get out of date with the source code.

As such, I consider OpenAPI support to be an extremely important (if not a non-negotiable) component of any web server framework. 