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
                    res.status_code(StatusCode::OK);
                    // TODO return auth token bundled with expiration time in a JSON blob
                    //   so the user has visibility into when their token expires
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