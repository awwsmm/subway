use crate::auth::{Authenticator, Token};
use salvo::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub(crate) struct Auth {
    roles: Vec<String>, // roles allowed to access this route
}

impl Auth {
    pub(crate) fn new(roles: &[&str]) -> Self {
        Self { roles: roles.iter().map(|s| s.to_string()).collect() }
    }
}

#[async_trait]
impl Handler for Auth {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, _ctrl: &mut FlowCtrl) {

        let user = match req.header::<&str>("x-token") {
            None => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render("Missing or malformed x-token header");
                return;
            }

            Some(token_header) => {
                let token = Token::new(String::from(token_header));
                let state = depot.obtain::<Arc<Mutex<Authenticator>>>().unwrap();
                let mut auth = state.lock().await;
                auth.get_user(token)
            }
        };

        match user {
            Some(user) if user.roles.iter().any(|role| self.roles.contains(role)) => {
                // TODO here we should add the roles, "sub", "preferred_username", etc. to the Depot
                depot.insert("token_user_name", user.name);
                depot.insert("token_user_id", user.id);
            }

            None => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render("Unrecognized authentication token");
            }

            _ => {
                res.status_code(StatusCode::UNAUTHORIZED);
                res.render("User is missing required role");
            }
        }
    }
}
