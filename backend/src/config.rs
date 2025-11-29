use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
pub(crate) struct DBConfig {
    pub(crate) mode: String,
    pub(crate) url: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AuthConfig {
    pub(crate) mode: String,
}

#[derive(Debug, Deserialize)]
/// This application configuration is parsed from the `config.toml` file.
pub(crate) struct Config {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) cors_allowlist: Vec<String>,
    pub(crate) tls_certificate_path: String,
    pub(crate) log_level: String,
    pub(crate) tls_key_path: String,
    pub(crate) db: DBConfig,
    pub(crate) auth: AuthConfig,
}

impl Config {
    pub(crate) fn new(toml_file: &str) -> Self {

        // panic if we cannot read the app config file
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
            },
            cors_allowlist: Self::parse_cors_allowlist(config.cors_allowlist),
            tls_certificate_path: env::var("SUBWAY_TLS_CERTIFICATE_PATH").unwrap_or(config.tls_certificate_path),
            tls_key_path: env::var("SUBWAY_TLS_KEY_PATH").unwrap_or(config.tls_key_path),
            log_level: env::var("RUST_LOG").unwrap_or(config.log_level),
            db: DBConfig {
                mode: env::var("SUBWAY_DB_MODE").unwrap_or(config.db.mode),
                url: env::var("SUBWAY_DB_URL").unwrap_or(config.db.url),
            },
            auth: AuthConfig {
                mode: env::var("SUBWAY_AUTH_MODE").unwrap_or(config.auth.mode),
            }
        }
    }

    fn parse_cors_allowlist(cors_allowlist: Vec<String>) -> Vec<String> {
        env::var("SUBWAY_CORS_ALLOWLIST").map(|s| s.split(',').map(|e| e.to_owned()).collect())
            .unwrap_or(cors_allowlist)
    }
}
