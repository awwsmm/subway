mod db;
mod model;
mod handlers;
mod auth_middleware;
mod config;
mod auth;
mod newdb;

use crate::auth::Authenticator;
use crate::auth_middleware::Auth;
use crate::config::Config;
use crate::db::Database;
use salvo::catcher::Catcher;
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;
use salvo_extra::affix_state;
use std::sync::Arc;
use tokio::sync::Mutex;

// There should be no endpoint definitions here. The purpose of main.rs is just to wire up the
// endpoint implementations, which themselves live in different files.

fn read_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting subway-backend...");

    // TODO (config) move this file path to config
    let cert_file_path = "../certs/cert.pem";
    let cert = match read_file(cert_file_path) {
        Ok(string) => string,
        Err(e) => panic!("unable to read TLS certificate file: {}", e)
    };

    // TODO (config) move this file path to config
    let key_file_path = "../certs/key.pem";
    let key = match read_file(key_file_path) {
        Ok(string) => string,
        Err(e) => panic!("unable to read TLS key file: {}", e)
    };

    log::debug!("loaded TLS certificate and key files");

    let tls_config = RustlsConfig::new(Keycert::new().cert(cert).key(key));
    log::debug!("configured TLS");

    let config = Config::new("config.toml");
    log::info!("loaded config: {:?}", config);

    let host_port = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::new(host_port.clone()).rustls(tls_config.clone());
    log::debug!("created TCP listener and bound to {}", host_port);

    // TODO (config) move this host and port to config
    let acceptor = QuinnListener::new(tls_config.build_quinn_config().unwrap(), ("0.0.0.0", 5800))
        .join(listener)
        .bind()
        .await;
    log::debug!("created QUIC listener and bound to {}", host_port);

    // TODO import regex package and enable this
    // PathFilter::register_wisp_regex(
    //     "guid",
    //     Regex::new("[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}").unwrap(),
    // );

    // TODO (config) pull CORS configuration out into config.toml
    let origins = ["http://localhost:5173"];

    // TODO (best practices) research and implement best practices for CORS here
    let cors = Cors::new()
        .allow_origin(origins) // Allow specific origins
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE]) // Allow specific HTTP methods
        // .allow_headers(vec!["Content-Type".into(), "Authorization".into()]) // Allow specific headers
        .allow_credentials(true) // Allow sending of cookies and authentication headers
        .max_age(86400) // Cache preflight requests for 24 hours
        .into_handler();

    log::debug!("configured CORS");

    /*
    // Endpoints always use the nouns from /model, as plural.
    //   reproduced from: https://stackoverflow.com/a/32257339/2925434
    //   For multiple resource items:
    //
    //     GET    /resources - returns a list of resource items
    //     POST   /resources - creates one or many resource items
    //     PUT    /resources - updates one or many resource items
    //     PATCH  /resources - partially updates one or many resource items
    //     DELETE /resources - deletes all resource items
    //
    //   And for single resource items:
    //
    //     GET    /resources/:id - returns a specific resource item based on :id parameter
    //     POST   /resources/:id - creates one resource item with specified id (requires validation)
    //     PUT    /resources/:id - updates a specific resource item
    //     PATCH  /resources/:id - partially updates a specific resource item
    //     DELETE /resources/:id - deletes a specific resource item
    //
    //   In Rust code, the handler files should be named accordingly, with nouns first
    //
    //     GET  /posts     => handlers/posts/get.rs (fn many())
    //     GET  /posts/:id => handlers/posts/get.rs (fn one())
    //     POST /posts     => handlers/posts/post.rs (fn many(), fn one())
    //     etc.
    //
    //   Not all endpoints will necessarily be implemented for each model type.
    //
    //   The :id parameter (or whatever it is) should always be the primary key of the table which
    //   stores those objects. To filter at the database level, use query parameters
    //
    //     GET /posts?published_after=2025-09-09

    // See https://salvo.rs/guide/concepts/router.html#extracting-parameters-from-routes
    //   to learn about extracting parameters from routes using Salvo
    //
    // See https://salvo.rs/guide/concepts/request#retrieving-query-parameters
    //   to learn about extracting query parameters
    */

    let public_router = Router::new()
        .hoop(affix_state::inject(Arc::new(Mutex::new(Database::new(config.db.mode.as_ref(), config.db.url.as_ref()))))) // add DB to state
        .hoop(affix_state::inject(Arc::new(Mutex::new(Authenticator::new(config.auth.mode.as_str()))))) // add auth to state
        // TODO preface all of these with /v0/ before pushing to production for the first time
        .push(Router::with_path("hello").get(handlers::misc::hello::hello))
        .push(Router::with_path("posts").get(handlers::posts::get::many))
        .push(Router::with_path("posts/{id}").get(handlers::posts::get::one))
        .push(Router::with_path("health").get(handlers::health::check))
        ;

    log::debug!("created router");

    // TODO consider replacing env!("CARGO_PKG_VERSION") with clap's crate_version macro
    let doc = OpenApi::new("test api", env!("CARGO_PKG_VERSION")).merge_router(&public_router);

    let public_router_with_openapi = public_router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("api-doc"));

    // 404.html
    let catcher = Catcher::default().hoop(handlers::misc::not_found::not_found);

    log::debug!("added OpenAPI docs UI to router");

    Server::new(acceptor).serve(
        Service::new(
            public_router_with_openapi
                .hoop(cors) // Apply the CORS middleware globally
                .push({ // login flows

                    let router = Router::new()
                        .push(Router::with_path("login").post(handlers::login::username_and_password::login));

                    // TODO parse auth.mode string to an AuthMode _once_, above, and panic up there instead of down here
                    match config.auth.mode.as_str() {
                        "keycloak" => router
                            .push(Router::with_path("login-keycloak").get(handlers::login::keycloak_token::login)),
                        "in-memory" => router,
                        _ => panic!("unsupported authentication mode: {}", config.auth.mode),
                    }

                })
                .push(
                    Router::with_path("posts")
                        .hoop(Auth::new(&["user"]))
                        .post(handlers::posts::post::many)
                )
                .push(
                    // this is an admin-only route
                    Router::with_path("/admin-only")
                        .hoop(Auth::new(&["admin"]))
                        .get(handlers::misc::admin_only::admin_only)
                )
                .push(
                    // this is a user-only route
                    Router::with_path("/user-only")
                        .hoop(Auth::new(&["user"]))
                        .get(handlers::misc::user_only::user_only)
                )
        ).catcher(catcher)
    ).await;
}