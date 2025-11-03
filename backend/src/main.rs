mod db;
mod model;
mod handlers;
mod auth_middleware;
mod config;
mod auth;

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

#[tokio::main]
async fn main() {

    let cert = include_bytes!("../certs/cert.pem").to_vec();
    let key = include_bytes!("../certs/key.pem").to_vec();
    println!("Loaded certs");

    let tls_config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));
    println!("Configured TLS");

    let config = Config::new("config.toml");
    println!("Loaded config: {:?}", config);

    let host_port = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::new(host_port.clone()).rustls(tls_config.clone());
    println!("Created TCP Listener");

    // Create QUIC listener and combine with TCP listener
    let acceptor = QuinnListener::new(tls_config.build_quinn_config().unwrap(), ("0.0.0.0", 5800))
        .join(listener)
        .bind()
        .await;
    println!("Created QUIC listener and bound to {}", host_port);

    // TODO import regex package and enable this
    // PathFilter::register_wisp_regex(
    //     "guid",
    //     Regex::new("[0-9a-fA-F]{8}-([0-9a-fA-F]{4}-){3}[0-9a-fA-F]{12}").unwrap(),
    // );

    // TODO pull CORS configuration out into config.toml
    let origins = ["http://localhost:5173"];

    let cors = Cors::new()
        .allow_origin(origins) // Allow specific origins
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE]) // Allow specific HTTP methods
        // .allow_headers(vec!["Content-Type".into(), "Authorization".into()]) // Allow specific headers
        .allow_credentials(true) // Allow sending of cookies and authentication headers
        .max_age(86400) // Cache preflight requests for 24 hours
        .into_handler();

    println!("Configured CORS");

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

    let public_router = Router::new()
        .hoop(affix_state::inject(Arc::new(Mutex::new(Database::new(config.db.mode.as_ref(), config.db.url.as_ref()))))) // add DB to state
        .hoop(affix_state::inject(Arc::new(Mutex::new(Authenticator::new(config.auth.mode.as_str()))))) // add auth to state
        .push(Router::with_path("hello").get(handlers::misc::hello::hello))
        .push(Router::with_path("posts").get(handlers::posts::get::many))
        .push(Router::with_path("posts/{id}").get(handlers::posts::get::one))
        .push(Router::with_path("health").get(handlers::health::check))
        ;

    println!("Created router");

    // TODO consider replacing env!("CARGO_PKG_VERSION") with clap's crate_version macro
    let doc = OpenApi::new("test api", env!("CARGO_PKG_VERSION")).merge_router(&public_router);

    let public_router = public_router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    let catcher = Catcher::default().hoop(handlers::misc::not_found::not_found);

    println!("Added OpenAPI docs");

    Server::new(acceptor).serve(
        Service::new(
            public_router
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