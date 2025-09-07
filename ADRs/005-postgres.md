Architectural Decision Record No. ADR-005

# Why Postgres?

There are many different kinds of databases.

To start, we must pick between a relational ("SQL") or non-relational ("NoSQL") DB. As I anticipate that this project will mainly be concerned with rigid-schema information (blog post data and metadata), I think a relational database is probably fine.

Among the many relational DBs, I have the most personal experience with MySQL and Postgres. Salvo's GitHub declares that it supports databases "including SQLite, PostgreSQL, and MySQL via SQLx, SeaORM, Diesel, Rbatis". The former three are databases, and the latter four are Object-Relational Mappings (ORMs), which allow for the "translation" between the data models in the database and data structures in the programming language of choice.

I started this project with Postgres, and have not yet encountered any issues which have caused me to question that choice. However, in case we need to swap out external services in the future, they should all exist behind interfaces. As of this writing, there is a `trait DatabaseLike` in `db.rs`...

```rust
pub(crate) trait DatabaseLike {
    async fn connect() -> Result<(), String>;

    async fn add_user(&mut self, id: i32, name: String) -> Result<(), String>;

    async fn get_user(&self, id: i32) -> Result<User, String>;
}
```

...which is implemented by a `postgres` database and also by an `in_memory` database.

In general, "programming to an interface" like this is a pattern that should be followed throughout this project, as much as possible.