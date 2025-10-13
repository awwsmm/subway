use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use salvo::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize, Clone)]
struct Roles {
    roles: Vec<String>,
}

// example decoded access_token
//
// {
//   "exp": 1760359940,
//   "iat": 1760359640,
//   "jti": "onrtro:e592c0a2-368e-79c2-3c35-026b40dda768",
//   "iss": "http://localhost:8989/realms/myrealm",
//   "typ": "Bearer",
//   "azp": "my-confidential-client",
//   "sid": "7e675b31-9841-44e5-b4a4-44ae42bca6ca",
//   "resource_access": {
//     "my-confidential-client": {
//       "roles": [
//         "client-user"
//       ]
//     }
//   },
//   "scope": "openid profile",
//   "name": "Bob User",
//   "preferred_username": "bob",
//   "given_name": "Bob",
//   "family_name": "User"
// }
#[derive(Debug, Deserialize, Clone)]
struct AccessToken {
    exp: usize, // expiry time (UNIX timestamp)
    iss: String, // the issuer of the token, should be: http://localhost:8989/realms/myrealm
    azp: String, // authorized party (the client / app acting on behalf of the user), should be: my-confidential-client
    resource_access: HashMap<String, Roles>, // map of client names to lists of roles
    preferred_username: String, // the user's (mutable) username
}

// example decoded id_token
//
// {
//   "exp": 1760359940,
//   "iat": 1760359640,
//   "jti": "a0019df9-7370-0e74-c316-a613a6fc9783",
//   "iss": "http://localhost:8989/realms/myrealm",
//   "aud": "my-confidential-client",
//   "sub": "7f16300f-6063-41ef-9428-ced32ef5adad",
//   "typ": "ID",
//   "azp": "my-confidential-client",
//   "sid": "7e675b31-9841-44e5-b4a4-44ae42bca6ca",
//   "at_hash": "uHUl9PtVRuABezMMlDfjLQ",
//   "name": "Bob User",
//   "preferred_username": "bob",
//   "given_name": "Bob",
//   "family_name": "User"
// }
#[derive(Debug, Deserialize, Clone)]
pub struct IdToken {
    exp: usize, // expiry time (UNIX timestamp)
    iss: String, // the issuer of the token, should be: http://localhost:8989/realms/myrealm
    aud: String, // audience (the client / app acting on behalf of the user), should be: my-confidential-client
    sub: String, // the subject of the token (whom the token refers to), the user's UUID
    azp: String, // authorized party (the client / app acting on behalf of the user), should be: my-confidential-client
    preferred_username: String, // the user's (mutable) username
}

#[derive(Clone)]
pub struct KeycloakAuth {
    roles: Vec<String>, // roles allowed to access this route
}

impl KeycloakAuth {
    pub fn new(roles: &[&str]) -> Self {
        Self { roles: roles.iter().map(|s| s.to_string()).collect() }
    }
}

#[async_trait]
impl Handler for KeycloakAuth {
    async fn handle(&self, req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
        let access_token_header = req.headers().get("x-keycloak-access-token").and_then(|v| v.to_str().ok());
        let id_token_header = req.headers().get("x-keycloak-id-token").and_then(|v| v.to_str().ok());
        let realm_header = req.headers().get("x-keycloak-realm").and_then(|v| v.to_str().ok());

        for header in req.headers() {
            println!("header: {:?}", header);
        }

        if access_token_header.is_none() {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render("Missing or malformed keycloak_access_token header");

        } else if id_token_header.is_none() {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render("Missing or malformed keycloak_id_token header");

        } else if realm_header.is_none() {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render("Missing or malformed keycloak_realm header");

        } else {
            let access_token = access_token_header.unwrap();
            let id_token = id_token_header.unwrap();
            let realm = realm_header.unwrap();

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

            let access_token_validation = Validation::new(Algorithm::RS256);

            let maybe_access_data = decode::<AccessToken>(access_token, &decoding_key, &access_token_validation);

            let client = "my-confidential-client";

            let mut id_token_validation = Validation::new(Algorithm::RS256);
            id_token_validation.set_audience(&[client]);

            let maybe_id_data = decode::<IdToken>(id_token, &decoding_key, &id_token_validation);

            match maybe_access_data {
                Ok(access_token_data) => {

                    println!("access token data: {:?}", access_token_data);
                    println!("id token data: {:?}", maybe_id_data);

                    if access_token_data.claims.resource_access.get(client).map(|roles| roles.roles.iter().any(|role| self.roles.contains(&role))).unwrap_or(false) {

                        // TODO here we should add the roles, "sub", "preferred_username", etc. to the Depot
                        depot.insert("username", access_token_data.claims.preferred_username);

                        return; // Allow request to continue

                    } else {
                        res.status_code(StatusCode::UNAUTHORIZED);
                        res.render("User does not have required role(s) to access this resource");
                    }
                }
                Err(err) => {
                    res.status_code(StatusCode::UNAUTHORIZED);
                    res.render(format!("Invalid token: {err}"));
                    return;
                }
            }
        }
    }
}
