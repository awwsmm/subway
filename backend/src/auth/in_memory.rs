use crate::auth::{AuthenticatorLike, AuthenticatorState, Token, User};
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub(crate) struct Authenticator {
    state: AuthenticatorState,
}

impl Authenticator {
    pub(in crate::auth) fn new() -> Self {
        Self { state: AuthenticatorState::new() }
    }
}

impl AuthenticatorLike for Authenticator {
    async fn login(&mut self, username: String, password: String) -> Result<Token, String> {

        // here, we need to read realm-export.json and pull user info from there

        // TODO inject RealmExport to make this method unit-testable?
        let file_path = "../keycloak/realm-export.json";
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);

        let realm_export: realm_export::RealmExport = serde_json::from_reader(reader).unwrap();

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

                        let user = User {
                            name: user.username.clone(),
                            id: Uuid::new_v3(&Uuid::NAMESPACE_DNS, &hasher.finish().to_be_bytes()),
                            roles: user.realm_roles.clone(),
                            // TODO parameterize token lifetime, currently hard-coded to 30 seconds
                            expires_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 30,
                        };

                        Ok(self.state.add_user(user))
                    }
                }
            }
        }
    }

    fn get_user(&mut self, token: Token) -> Option<User> {
        self.state.get_user(token)
    }
}

mod realm_export {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub(in crate::auth) struct RealmExport {
        // #[serde(rename = "realm")]
        // realm: String,

        // #[serde(rename = "accessTokenLifespan")]
        // access_token_lifespan: u64,

        // #[serde(rename = "ssoSessionIdleTimeout")]
        // sso_session_idle_timeout: u64,

        // #[serde(rename = "sslRequired")]
        // ssl_required: String,

        // enabled: bool,

        // clients: Vec<Client>,

        // roles: Roles,

        pub(in crate::auth) users: Vec<User>,
    }

    // #[derive(Debug, Serialize, Deserialize)]
    // struct Client {
    //     #[serde(rename = "clientId")]
    //     client_id: String,
    //
    //     enabled: bool,
    //
    //     #[serde(default)]
    //     protocol: Option<String>,
    //
    //     #[serde(rename = "publicClient")]
    //     public_client: bool,
    //
    //     #[serde(default)]
    //     secret: Option<String>,
    //
    //     #[serde(rename = "redirectUris")]
    //     redirect_uris: Vec<String>,
    //
    //     #[serde(rename = "defaultClientScopes")]
    //     default_client_scopes: Vec<String>,
    //
    //     #[serde(rename = "optionalClientScopes")]
    //     optional_client_scopes: Vec<String>,
    //
    //     #[serde(rename = "clientAuthenticatorType")]
    //     #[serde(default)]
    //     client_authenticator_type: Option<String>,
    //
    //     #[serde(rename = "directAccessGrantsEnabled")]
    //     #[serde(default)]
    //     direct_access_grants_enabled: Option<bool>,
    //
    //     #[serde(rename = "defaultRoles")]
    //     default_roles: Vec<String>,
    // }

    // #[derive(Debug, Serialize, Deserialize)]
    // struct Roles {
    //     realm: Vec<Role>,
    // }

    // #[derive(Debug, Serialize, Deserialize)]
    // struct Role {
    //     id: String,
    //     name: String,
    //     description: String,
    // }

    #[derive(Deserialize, Hash)]
    pub(in crate::auth) struct User {
        pub(in crate::auth) username: String,
        // enabled: bool,

        // #[serde(rename = "emailVerified")]
        // email_verified: bool,

        // #[serde(rename = "firstName")]
        // first_name: String,

        // #[serde(rename = "lastName")]
        // last_name: String,

        // email: String,

        #[serde(rename = "realmRoles")]
        pub(in crate::auth) realm_roles: Vec<String>,

        pub(in crate::auth) credentials: Vec<Credential>,
    }

    #[derive(Deserialize, Hash)]
    pub(in crate::auth) struct Credential {
        #[serde(rename = "type")]
        pub(in crate::auth) cred_type: String,

        pub(in crate::auth) value: String,

        // temporary: bool,
    }

}