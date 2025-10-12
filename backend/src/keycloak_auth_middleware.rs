use std::collections::HashMap;
use jsonwebtoken::{decode, decode_header, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation};
use reqwest::Client;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    // pub sub: String,
    // pub email: Option<String>,
    // pub preferred_username: Option<String>,
    pub exp: usize,
    // pub realm_access: Option<RealmAccess>,
    // Add more fields as needed
}

#[derive(Debug, Deserialize, Clone)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Clone)]
pub struct KeycloakAuth {
    validation: Validation,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: i64,
    pub session_state: String,
    pub scope: String,
}

impl KeycloakAuth {
    pub fn new() -> Self {
        let mut validation = Validation::new(Algorithm::RS256);

        Self {
            validation: validation.clone(),
        }
    }
}

#[async_trait]
impl Handler for KeycloakAuth {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let auth_header = req.headers().get("authorization").and_then(|v| v.to_str().ok());

        if let Some(auth_header) = auth_header {
            if let Some(access_token) = auth_header.strip_prefix("Bearer ") {

                let realm = req.header::<String>("x-realm").unwrap();

                let client = Client::new();

                let header = decode_header(access_token).unwrap();
                println!("looking for kid: {:?}", header.kid);

                let jwk_url = format!("http://keycloak_container:8080/realms/{}/protocol/openid-connect/certs", realm);

                let jwks: serde_json::Value = client
                    .get(jwk_url)
                    .bearer_auth(access_token)
                    .send().await.unwrap().json().await.unwrap();

                println!("JWKs found: {:?}", jwks);

                // Use the first key from the JWKs (in real use, you should match "kid")
                let jwk = jwks["keys"].as_array().unwrap().iter().find(|arr| arr.as_object().unwrap().get("kid").unwrap().as_str().unwrap() == header.kid.clone().unwrap()).unwrap();

                let n = jwk["n"].as_str().unwrap();
                let e = jwk["e"].as_str().unwrap();
                let decoding_key = Arc::new(DecodingKey::from_rsa_components(n, e).unwrap());

                match decode::<Claims>(access_token, &decoding_key, &self.validation) {
                    Ok(token_data) => {
                        println!("Token data: {:?}", token_data);
                        return; // Allow request to continue
                    }
                    Err(err) => {
                        res.status_code(StatusCode::UNAUTHORIZED);
                        res.render(format!("Invalid token: {err}"));
                        return;
                    }
                }
            }
        }

        res.status_code(StatusCode::UNAUTHORIZED);
        res.render("Missing or malformed Authorization header");
    }
}
