use crate::auth::{Authenticator, AuthenticatorLike};
use reqwest::StatusCode;
use salvo::oapi::endpoint;
use salvo::{Depot, Request, Response};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[endpoint]
pub(crate) async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Authenticator>>>().unwrap();

    match req.parse_json::<Credentials>().await {
        Ok(credentials) => {
            let mut auth = state.lock().await;
            match auth.login(credentials.username, credentials.password).await {
                Ok(auth_token) => {

                    // at this point, I have
                    // - their username, their password, their user UUID, their roles
                    // - the expiry time of the token, all tokens and auth, etc.
                    //
                    // I can hold all of this info in memory until the Keycloak auth expires
                    // I just need to send them a random auth string that they send back to me
                    // that random auth string is the key of a key-value pair in a HashMap
                    // multiple users can be logged in at once
                    // if their random string does not exist in the map, they must not be logged in

                    res.status_code(StatusCode::OK);
                    // TODO return auth token bundled with expiration time in a JSON blob
                    res.render(auth_token);
                }
                Err(e) => {
                    res.status_code(StatusCode::UNAUTHORIZED);
                    res.render(format!("error logging in: {}", e))
                }
            }
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("error parsing request body: {}", e))
        },
    }
}