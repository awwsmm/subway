Architectural Decision Record No. ADR-007

# Resource files

In this project, there are many files which are not Rust source code, yet which are nevertheless integral to the operation of the application. For example, the `*.html` files in `resources/`. There is a `404.html` page here which is delivered to a user when they try to visit a page that does not exist. There is also the `migrations/` directory, which contains the scripts which set up the database and its tables.

While researching the idiomatic way to include these sorts of files in a Rust project, I came across [this Reddit thread](https://www.reddit.com/r/rust/comments/7d1vdm/whats_the_standard_way_of_bundling_resource_files/) with the following top answer

> You got three answers with three different solutions: An external build script (such as CMake), a build script that's used by Cargo, and `include_bytes!`. Which one is best depends mostly on how much processing these image files need.
>
> If there's no processing, you just want to include a file in the application, use `include_bytes!` for binary files (such as images) and `include_str!` for text files to be read as strings.
>
> If you want to do some light processing (maybe creating lower-res images from a high-res source), use a `build.rs` script. This is a normal rust program, where you can use Rust libraries and/or call external commands.
>
> If the processing is complex or time-consuming, so that you want the Make-like functionality of only re-running those tasks that are outdated, you may want to use an external tool. Plain Make, CMake, or really just a script in any scripting language that processed all the data file, and finally runs `cargo build` at the end.
>
> In your case, there seems to be no processing involved, so go with `include_bytes!`.

As the `resources/` files are meant to simply be sent back to the user as-is, the most appropriate solution here would seem to be to use `include_str!`. However, we also use [`diesel_migrations`](006-diesel.md) in this project, and that library requires `*.sql` files in a `migrations/` directory. In order for `diesel` to work correctly, we need to make sure those files are on the same filessystem as the Rust source code at compile time.

Since [docker](002-docker.md) is used to build the project, and since we use a multi-stage build (which first copies the source code and builds the project, and then copies the binary to the container image), we need to `COPY` the `migrations/` during the build phase.

In order to reduce the mental overhead of maintaining the different approaches, therefore, I've opted to use `COPY` for all directories of resource files. They are all copied into the container image in the [Dockerfile](../Dockerfile).

If one wanted to compile the application binary and run it outside a Docker environment (not recommended), care would have to be taken to make sure that the `resources/` directory (and any others, since this was written) are also available, with the correct set of files.

Note that the downside of this approach is that these resource files are then read from the filesystem each time they are needed. If this becomes a performance issue, consider using `include_str!` or `include_bytes!` to embed these files directly in the binary.