use crate::auth::Authenticator;
use reqwest::StatusCode;
use salvo::oapi::endpoint;
use salvo::{Depot, Request, Response};
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;

#[endpoint]
pub(crate) async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Authenticator>>>().unwrap();

    let access_token = match req.header::<&str>("x-keycloak-access-token") {
        Some(header) => header,
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("missing or invalid x-keycloak-access-token header");
            return;
        }
    };

    let id_token = match req.header::<&str>("x-keycloak-id-token") {
        Some(header) => header,
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("missing or invalid x-keycloak-id-token header");
            return;
        }
    };

    let realm = match req.header::<&str>("x-keycloak-realm") {
        Some(header) => header,
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("missing or invalid x-keycloak-realm header");
            return;
        }
    };

    let mut auth = state.lock().await;

    let auth = match auth.deref_mut() {
        Authenticator::InMemory(_) => panic!("cannot use in-memory authentication with /login-keycloak endpoint"),
        Authenticator::Keycloak(auth) => auth,
    };

    match auth.login_with_tokens(access_token, id_token, realm).await {
        Ok(auth_token) => {
            res.status_code(StatusCode::OK);
            res.render(auth_token);
        }
        Err(e) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(format!("error logging in: {}", e))
        }
    }
}