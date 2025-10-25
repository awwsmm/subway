use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) host: String,
    pub(crate) port: u16,
}

impl Config {
    pub(crate) fn new(toml_file: &str) -> Self {
        let toml_content = fs::read_to_string(toml_file)
            .expect(format!("Failed to read {}", toml_file).as_str());

        let config: Config = toml::from_str(&toml_content)
            .expect("Failed to parse TOML");

        // override with env vars
        Config {
            host: env::var("SUBWAY_HOST").unwrap_or(config.host),
            port: match env::var("SUBWAY_PORT").map(|s| s.parse::<u16>()) {
                Ok(Ok(port)) => port,
                _ => config.port
            }
        }
    }
}



