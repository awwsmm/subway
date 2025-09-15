Architectural Decision Record No. ADR-001

# Why a REST Web Server?

As of this writing, REST is the conventional web API design pattern. We could consider adding a GraphQL layer on top of the database in the future, but REST is the easiest to get up and running with.

I wouldn't add gRPC / Protobuf endpoints, as these are used primarily for server-to-server communication, and not very user-friendly.

We should aim to follow best practices for REST API design as much as possible. See https://cloud.google.com/apis/design for more information.