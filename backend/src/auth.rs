mod realm_export;

use crate::auth::realm_export::RealmExport;
use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use rand::prelude::*;
use reqwest::ClientBuilder;
use salvo::{Response, Scribe};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::BufReader;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub(crate) struct User {
    pub(crate) name: String,
    pub(crate) id: Uuid,
    pub(crate) roles: Vec<String>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) struct Token(String);

impl Scribe for Token {
    fn render(self, res: &mut Response) {
        res.render(self.0);
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Roles {
    roles: Vec<String>,
}

// example decoded access_token
//
// {
//   "exp": 1760906329,
//   "iat": 1760906029,
//   "jti": "onrtro:61b2bdd0-d403-9f89-5e99-cea205794395",
//   "iss": "https://localhost/realms/myrealm",
//   "typ": "Bearer",
//   "azp": "my-confidential-client",
//   "sid": "652809cd-9f35-492e-b358-f040bf4dd3b1",
//   "realm_access": {
//     "roles": [
//       "user"
//     ]
//   },
//   "scope": "openid profile",
//   "name": "Bob User",
//   "preferred_username": "bob",
//   "given_name": "Bob",
//   "family_name": "User"
// }
#[derive(Debug, Deserialize, Clone)]
struct AccessToken {
    // exp: usize, // expiry time (UNIX timestamp)
    // iss: String, // the issuer of the token, should be: https://localhost/realms/myrealm
    // azp: String, // authorized party (the client / app acting on behalf of the user), should be: my-confidential-client
    realm_access: Roles, // list of roles in the realm
    preferred_username: String, // the user's (mutable) username
}

// example decoded id_token
//
// {
//   "exp": 1760359940,
//   "iat": 1760359640,
//   "jti": "a0019df9-7370-0e74-c316-a613a6fc9783",
//   "iss": "https://localhost/realms/myrealm",
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
    // exp: usize, // expiry time (UNIX timestamp)
    // iss: String, // the issuer of the token, should be: https://localhost/realms/myrealm
    // aud: String, // audience (the client / app acting on behalf of the user), should be: my-confidential-client
    sub: String, // the subject of the token (whom the token refers to), the user's UUID
    // azp: String, // authorized party (the client / app acting on behalf of the user), should be: my-confidential-client
    // preferred_username: String, // the user's (mutable) username
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

pub(crate) struct Authenticator {
    pub(crate) mode: String,
    rand: StdRng,
    state: HashMap<Token, User>, // TODO replace with a cache? Redis? ValKey?
}

impl Authenticator {

    pub(crate) fn new(mode: &str) -> Self {
        Self {
            mode: String::from(mode),
            rand: StdRng::from_os_rng(), // https://rust-random.github.io/book/guide-rngs.html
            state: HashMap::new(),
        }
    }

    pub(crate) async fn login(&mut self, username: String, password: String) -> Result<Token, String> {
        if self.mode == "keycloak" {

            // TODO FIXME -- here, we need to call Keycloak to get access token, id token, etc.
            // base this off of what's currently happening in keycloak_auth_middleware.rs

            // export KC_UNAME="bob"; export KC_PWD=$KC_UNAME; \
            //  eval $(curl -k -X POST https://localhost/realms/myrealm/protocol/openid-connect/token \
            //   -d "client_id=my-confidential-client" \
            //   -d "client_secret=my-client-secret" \
            //   -d "grant_type=password" \
            //   -d "username=$KC_UNAME" \
            //   -d "password=$KC_PWD" \
            //   -d "scope=openid"

            // TODO host name and port and realm here should be env vars
            let url = "https://subway-keycloak:8443/realms/myrealm/protocol/openid-connect/token";

            let client = ClientBuilder::new()
                .danger_accept_invalid_certs(true) // TODO FIXME do not use in production
                .build()
                .unwrap();

            let params = [
                ("client_id", String::from("my-confidential-client")),
                ("client_secret", String::from("my-client-secret")),
                ("grant_type", String::from("password")),
                ("username", username),
                ("password", password),
                ("scope", String::from("openid")),
            ];

            let response: serde_json::Value = client
                .post(url)
                .form(&params)
                .send().await.unwrap().json().await.unwrap();

            let access_token_header = response["access_token"].as_str();
            let id_token_header = response["id_token"].as_str();
            let realm_header = Some("myrealm"); // TODO parameterize

            println!("header: {:?}", access_token_header);
            println!("header: {:?}", id_token_header);
            println!("header: {:?}", realm_header);

            if access_token_header.is_none() {
                Err(String::from("Missing or malformed keycloak_access_token header"))

            } else if id_token_header.is_none() {
                Err(String::from("Missing or malformed keycloak_id_token header"))

            } else if realm_header.is_none() {
                Err(String::from("Missing or malformed v header"))

            } else {
                let access_token = access_token_header.unwrap();
                let id_token = id_token_header.unwrap();
                let realm = realm_header.unwrap();

                // let client = Client::new();

                let header = decode_header(access_token).unwrap();
                println!("looking for kid: {:?}", header.kid);

                // TODO container name and port here should be env vars
                let jwk_url = format!("https://subway-keycloak:8443/realms/{}/protocol/openid-connect/certs", realm);

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

                match (maybe_access_data, maybe_id_data) {
                    (Ok(access_token_data), Ok(id_token_data)) => {

                        let user = User {
                            name: access_token_data.claims.preferred_username,
                            id: id_token_data.claims.sub.parse().unwrap(),
                            roles: access_token_data.claims.realm_access.roles.clone(),
                        };

                        let token = self.generate_token(32);

                        println!("inserting {:?} -> {:?}", token.clone(), user);

                        self.state.insert(token.clone(), user);

                        Ok(token)
                    }
                    _ => {
                        Err(String::from("invalid tokens"))
                    }
                }
            }

        } else {

            // here, we need to read realm-export.json and pull user info from there

            let file_path = "../keycloak/realm-export.json";
            let file = File::open(file_path).unwrap();
            let reader = BufReader::new(file);

            let realm_export: RealmExport = serde_json::from_reader(reader).unwrap();

            match realm_export.users.iter().find(|&user| user.username == username) {
                None => Err("username or password incorrect".to_owned()),
                Some(user) => {
                    match user.credentials.iter().find(|&cred|
                        cred.cred_type == "password" && cred.value == password
                    ) {
                        None => Err("username or password incorrect".to_owned()),
                        Some(_) => {

                            // fake deterministic UUIDs for sample data
                            let mut hasher = DefaultHasher::new();
                            user.hash(&mut hasher);
                            let bytes = hasher.finish().to_be_bytes();
                            let id = Uuid::new_v3(&Uuid::NAMESPACE_DNS, &bytes);

                            let user = User {
                                name: user.username.clone(),
                                id,
                                roles: user.realm_roles.clone(),
                            };

                            let token = self.generate_token(32);

                            println!("inserting {:?} -> {:?}", token.clone(), user);

                            self.state.insert(token.clone(), user);

                            Ok(token)
                        }
                    }
                }
            }
        }
    }

    fn generate_token(&mut self, n_bytes: usize) -> Token {
        let mut random_bytes = vec![0u8; n_bytes];
        self.rand.fill_bytes(&mut random_bytes);
        Token(STANDARD.encode(&random_bytes))
    }

    fn get_user(&self, token: Token) -> Option<User> {
        self.state.get(&token).cloned()
    }
}