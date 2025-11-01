use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RealmExport {
    #[serde(rename = "realm")]
    pub(crate) realm: String,

    #[serde(rename = "accessTokenLifespan")]
    pub(crate) access_token_lifespan: u64,

    #[serde(rename = "ssoSessionIdleTimeout")]
    pub(crate) sso_session_idle_timeout: u64,

    #[serde(rename = "sslRequired")]
    pub(crate) ssl_required: String,

    pub(crate) enabled: bool,

    pub(crate) clients: Vec<Client>,

    pub(crate) roles: Roles,

    pub(crate) users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Client {
    #[serde(rename = "clientId")]
    pub(crate) client_id: String,

    pub(crate) enabled: bool,

    #[serde(default)]
    pub(crate) protocol: Option<String>,

    #[serde(rename = "publicClient")]
    pub(crate) public_client: bool,

    #[serde(default)]
    pub(crate) secret: Option<String>,

    #[serde(rename = "redirectUris")]
    pub(crate) redirect_uris: Vec<String>,

    #[serde(rename = "defaultClientScopes")]
    pub(crate) default_client_scopes: Vec<String>,

    #[serde(rename = "optionalClientScopes")]
    pub(crate) optional_client_scopes: Vec<String>,

    #[serde(rename = "clientAuthenticatorType")]
    #[serde(default)]
    pub(crate) client_authenticator_type: Option<String>,

    #[serde(rename = "directAccessGrantsEnabled")]
    #[serde(default)]
    pub(crate) direct_access_grants_enabled: Option<bool>,

    #[serde(rename = "defaultRoles")]
    pub(crate) default_roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Roles {
    pub(crate) realm: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Role {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub(crate) struct User {
    pub(crate) username: String,
    pub(crate) enabled: bool,

    #[serde(rename = "emailVerified")]
    pub(crate) email_verified: bool,

    #[serde(rename = "firstName")]
    pub(crate) first_name: String,

    #[serde(rename = "lastName")]
    pub(crate) last_name: String,

    pub(crate) email: String,

    #[serde(rename = "realmRoles")]
    pub(crate) realm_roles: Vec<String>,

    pub(crate) credentials: Vec<Credential>,
}

#[derive(Debug, Serialize, Deserialize, Hash)]
pub(crate) struct Credential {
    #[serde(rename = "type")]
    pub(crate) cred_type: String,

    pub(crate) value: String,

    pub(crate) temporary: bool,
}
