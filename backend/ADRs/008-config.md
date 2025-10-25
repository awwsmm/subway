Architectural Decision Record No. ADR-008

# Configuration

In the Rust world, there are a few options for parsing application configuration files

- [`serde`](https://github.com/serde-rs/serde) and [`toml`](https://github.com/toml-rs/toml)
- [`dotenvy`](https://github.com/allan2/dotenvy) (successor to unmaintained [`dotenv`](https://github.com/dotenv-rs/dotenv) crate)
- [`config`](https://github.com/rust-cli/config-rs)

...among others.

There are pros and cons to all of these

- `serde` / `toml` on their own do not allow for easily overriding the configuration file with env vars
- `dotenvy` is also now unmaintained, apparently
    - no new commits in months
    - no new version in years
    - `README.md` is out of date and examples do not work
- `config` professes to allow overriding config file values with env vars, but [this is broken](https://github.com/rust-cli/config-rs/issues/391)

The solution I've opted for is
1. parsing a `*.toml` file using `serde` / `toml`
2. manually overriding with env vars

This isn't a beautiful solution, but it works for now. Ideally, I'd like something like Lightbend's [HOCON](https://github.com/lightbend/config/blob/main/HOCON.md#hocon-human-optimized-config-object-notation). There are one or two Rust crate for HOCON, but they are either [unmaintained](https://crates.io/crates/hocon) or [nascent](https://crates.io/crates/hocon-rs).