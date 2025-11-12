use crate::auth::Authenticator;
use reqwest::StatusCode;
use salvo::oapi::endpoint;
use salvo::{Depot, Request, Response};
use std::ops::DerefMut;
use std::sync::Arc;
use tokio::sync::Mutex;

fn extract(header_name: &str, req: &Request, res: &mut Response) -> Option<String> {
    match req.header::<String>(header_name) {
        Some(header) => Some(header),
        _ => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("missing or invalid {} header", header_name));
            None
        }
    }
}

#[endpoint]
pub(crate) async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let state = depot.obtain::<Arc<Mutex<Authenticator>>>().unwrap();

    // TODO put 'sub' (keycloak user UUID) in access_token to make id token unnecessary
    // TODO put realm name in access_token to make realm token unnecessary
    let Some(access_token) = extract("x-keycloak-access-token", req, res) else { return };
    let Some(id_token) = extract("x-keycloak-id-token", req, res) else { return };
    let Some(realm) = extract("x-keycloak-realm", req, res) else { return };

    let mut auth = state.lock().await;

    let auth = match auth.deref_mut() {
        // TODO should never be able to get here with an InMemory Authenticator -- figure out how to make this invalid state unreachable
        Authenticator::InMemory(_) => panic!("cannot use in-memory authentication with /login-keycloak endpoint"),
        Authenticator::Keycloak(auth) => auth,
    };

    match auth.login_with_tokens(access_token.as_str(), id_token.as_str(), realm.as_str()).await {
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