Architectural Decision Record No. ADR-006

# Why Diesel?

Diesel is [one of several](https://salvo.rs/guide/topics/working-with-database) Object-Relational Mappings (ORMs) supported by Salvo. I chose Diesel because they were first on that list, not for any other particular reason.

Diesel provide an [async extension](https://docs.rs/diesel-async/latest/diesel_async/) and also a [database migration](https://docs.rs/diesel_migrations/latest/diesel_migrations/) extension, the latter of which I'm already using to automatically add tables to ephemeral databases.

If we encounter issues with performance or maintainability in the future, there's no reason why we shouldn't explore alternative ORMs. Let's try to keep all of that code behind the `DatabaseLike` `trait` so it doesn't leak into the rest of the application. (We could even explore having multiple Postgres implementations, one with Diesel and one with SQLx, for example.)

Note that the Diesel team compare their ORM to other popular ORMs like SQLx and SeaORM [on their website](https://diesel.rs/compare_diesel.html).