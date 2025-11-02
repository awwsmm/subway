use crate::auth::realm_export::RealmExport;
use crate::auth::{AuthenticatorLike, AuthenticatorState, Token, User};
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::BufReader;
use uuid::Uuid;

pub(crate) struct Authenticator {
    pub(in crate::auth) state: AuthenticatorState,
}

impl Authenticator {
    pub(in crate::auth) fn new() -> Self {
        Self { state: AuthenticatorState::new() }
    }
}

impl AuthenticatorLike for Authenticator {
    async fn login(&mut self, username: String, password: String) -> Result<Token, String> {

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

                        let token = self.state.generate_token(32);

                        println!("inserting {:?} -> {:?}", token.clone(), user);

                        self.state.map.insert(token.clone(), user);

                        Ok(token)
                    }
                }
            }
        }
    }
}