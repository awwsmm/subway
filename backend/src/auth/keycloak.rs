use crate::auth::{AuthenticatorLike, AuthenticatorState, Token, User};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, TokenData, Validation};
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::sync::Arc;


pub(crate) struct Authenticator {
    state: AuthenticatorState,
    client: Client,
}

impl Authenticator {
    pub(in crate::auth) fn new() -> Self {
        Self {
            state: AuthenticatorState::new(),
            client: ClientBuilder::new()
                .danger_accept_invalid_certs(true) // TODO FIXME do not use in production
                .build()
                .unwrap(),
        }
    }

    async fn decode_and_validate<T: DeserializeOwned + Clone>(
        &self,
        jwt: &str,
        realm: &str,
        validation: Validation,
    ) -> Result<TokenData<T>, String> {

        let header = decode_header(jwt).unwrap();

        // TODO container name and port and realm name here should be env vars
        let jwk_url = format!("https://subway-keycloak:8443/realms/{}/protocol/openid-connect/certs", realm);

        #[derive(Deserialize, Debug)]
        struct Key {
            kid: String,
            n: String,
            e: String,
        }

        #[derive(Deserialize, Debug)]
        struct JWKs {
            keys: Vec<Key>
        }

        let jwks = self.client.get(jwk_url).bearer_auth(jwt).send().await.unwrap().json::<JWKs>().await.unwrap();
        let jwk = jwks.keys.iter().find(|arr| arr.kid == header.kid.clone().unwrap()).unwrap();
        let decoding_key = Arc::new(DecodingKey::from_rsa_components(jwk.n.as_str(), jwk.e.as_str()).unwrap());

        // validates token, signature, and claims (exp, aud, iss)
        decode::<T>(jwt, &decoding_key, &validation).map_err(|e| e.to_string())
    }

    pub(crate) async fn login_with_tokens(
        &mut self,
        access_token: &str,
        id_token: &str,
        realm: &str
    ) -> Result<Token, String> {

        // validate token, signature, and claims (exp, aud, iss)

        log::debug!("validating access_token: {:?}", access_token);

        // TODO define list of known issuers and audiences in config, rather than hard-coding here
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&["https://localhost:8443/realms/myrealm"]);
        let maybe_access_data = self.decode_and_validate::<keycloak::AccessToken>(access_token, realm, validation).await;

        log::debug!("validating id_token: {:?}", id_token);

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["my-confidential-client"]);
        validation.set_issuer(&["https://localhost:8443/realms/myrealm"]);
        let maybe_id_data = self.decode_and_validate::<keycloak::IdToken>(id_token, realm, validation).await;

        match (maybe_access_data, maybe_id_data) {
            (Ok(access_token_data), Ok(id_token_data)) => {
                let user = keycloak::user_from(access_token_data.claims, id_token_data.claims);
                Ok(self.state.add_user(user))
            }
            _ => {
                Err(String::from("invalid tokens"))
            }
        }
    }
}

impl AuthenticatorLike for Authenticator {
    async fn login(&mut self, username: String, password: String) -> Result<Token, String> {

        // TODO host name and port and realm here should be env vars
        let url = "https://subway-keycloak:8443/realms/myrealm/protocol/openid-connect/token";

        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true) // TODO FIXME do not use in production
            .build()
            .unwrap();

        // TODO do away with this "direct access grant" pattern and use "Authorization Code Flow" instead
        let params = [
            ("client_id", String::from("my-confidential-client")),
            ("client_secret", String::from("my-client-secret")),
            ("grant_type", String::from("password")),
            ("username", username),
            ("password", password),
            // TODO clean up grants to avoid requiring both id and access tokens and this scope, below
            ("scope", String::from("openid")),
        ];

        #[derive(Deserialize)]
        struct Response {
            access_token: String,
            id_token: String,
        }

        match client.post(url).form(&params).send().await.unwrap().json::<Response>().await {
            // TODO fix this hard-coded realm, below
            Ok(r) => self.login_with_tokens(r.access_token.as_str(), r.id_token.as_str(), "myrealm").await,
            Err(e) => Err(format!("error parsing Keycloak response: {}", e)),
        }
    }

    fn get_user(&mut self, token: Token) -> Option<User> {
        self.state.get_user(token)
    }
}

mod keycloak {
    use crate::auth::User;
    use serde::Deserialize;
    use std::cmp::min;

    #[derive(Deserialize, Clone)]
    struct Roles {
        pub(in crate::auth::keycloak) roles: Vec<String>,
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
    #[derive(Deserialize, Clone)]
    pub(in crate::auth::keycloak) struct AccessToken {
        exp: u64, // expiry time (UNIX timestamp)
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
    #[derive(Deserialize, Clone)]
    pub(in crate::auth::keycloak) struct IdToken {
        exp: u64, // expiry time (UNIX timestamp)
        // iss: String, // the issuer of the token, should be: https://localhost/realms/myrealm
        // aud: String, // audience (the client / app acting on behalf of the user), should be: my-confidential-client
        sub: String, // the subject of the token (whom the token refers to), the user's UUID
        // azp: String, // authorized party (the client / app acting on behalf of the user), should be: my-confidential-client
        // preferred_username: String, // the user's (mutable) username
    }

    pub(in crate::auth::keycloak) fn user_from(access_token: AccessToken, id_token: IdToken) -> User {
        let expires_at = min(access_token.exp, id_token.exp);

        User {
            name: access_token.preferred_username,
            id: id_token.sub.parse().unwrap(),
            roles: access_token.realm_access.roles.clone(),
            expires_at,
        }
    }
}