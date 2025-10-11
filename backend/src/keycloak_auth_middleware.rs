use std::collections::HashMap;
use jsonwebtoken::{decode, errors::Error as JwtError, Algorithm, DecodingKey, TokenData, Validation};
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
    decoding_key: Arc<DecodingKey>,
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
    pub async fn from_jwk_url(jwk_url: &str, expected_aud: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new();

        let jwk_url = "http://keycloak_container:8080/realms/master/protocol/openid-connect/certs";

        // ---

        let mut params = HashMap::new();
        params.insert("grant_type", "password");
        params.insert("client_id", "admin-cli");
        params.insert("username", "kc_bootstrap_admin_username");
        params.insert("password", "kc_bootstrap_admin_password");

        let token_maybe = client
            .post("http://keycloak_container:8080/realms/master/protocol/openid-connect/token")
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        println!("token_maybe {:?}", token_maybe);

        // ---

        let jwks: serde_json::Value = client
            .get(jwk_url)
            .basic_auth("kc_bootstrap_admin_username", Some("kc_bootstrap_admin_password"))
            .send().await?.json().await?;

        println!("JWK found: {:?}", jwks);

        // Use the first key from the JWKs (in real use, you should match "kid")
        let jwk = jwks["keys"][0].clone();

        let n = jwk["n"].as_str().unwrap();
        let e = jwk["e"].as_str().unwrap();
        let decoding_key = Arc::new(DecodingKey::from_rsa_components(n, e)?);

        let mut validation = Validation::new(Algorithm::RS256);
        // validation.set_audience(&[expected_aud]);

        let result = Ok(Self {
            decoding_key: Arc::clone(&decoding_key),
            validation: validation.clone(),
        });

        println!("decoding...: {:?}", decode::<Claims>(token_maybe.access_token, &decoding_key, &validation)?);

        result
    }

    fn validate_token(&self, token: &str) -> Result<TokenData<Claims>, JwtError> {
        decode::<Claims>(token, &self.decoding_key, &self.validation)
    }
}

#[async_trait]
impl Handler for KeycloakAuth {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let auth_header = req.headers().get("authorization").and_then(|v| v.to_str().ok());

        if let Some(auth_header) = auth_header {
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                match self.validate_token(token) {
                    Ok(token_data) => {
                        req.extensions_mut().insert(token_data.claims);
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
