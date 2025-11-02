mod realm_export;
pub(crate) mod in_memory;
pub(crate) mod keycloak;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use rand::prelude::*;
use salvo::{Response, Scribe};
use std::collections::HashMap;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub(crate) struct User {
    pub(crate) name: String,
    pub(crate) id: Uuid,
    pub(crate) roles: Vec<String>,
    pub(crate) expires_at: u64, // UNIX timestamp
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) struct Token(String);

impl Scribe for Token {
    fn render(self, res: &mut Response) {
        res.render(self.0);
    }
}

impl Token {
    pub(crate) fn new(raw: String) -> Self {
        Self(raw)
    }
}

pub(crate) struct AuthenticatorState {
    rand: StdRng,
    map: HashMap<Token, User>, // TODO replace with a cache? Redis? ValKey?
}

impl AuthenticatorState {

    pub(crate) fn new() -> Self {
        Self {
            rand: StdRng::from_os_rng(), // https://rust-random.github.io/book/guide-rngs.html
            map: HashMap::new(),
        }
    }

    /// Generates a random base64 token to associate with user info saved in memory.
    fn generate_token(&mut self, n_bytes: usize) -> Token {
        let mut random_bytes = vec![0u8; n_bytes];
        self.rand.fill_bytes(&mut random_bytes);
        Token(STANDARD.encode(&random_bytes))
    }

    pub(crate) fn get_user(&mut self, token: Token) -> Option<User> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        match self.map.get(&token) {
            Some(user) if user.expires_at > now => {
                Some(user.clone())
            }
            _ => {
                self.map.remove(&token);
                None
            },
        }
    }
}

pub(crate) trait AuthenticatorLike {
    async fn login(&mut self, username: String, password: String) -> Result<Token, String>;
}

pub(crate) enum Authenticator {
    Keycloak(keycloak::Authenticator),
    InMemory(in_memory::Authenticator),
}

impl Authenticator {
    pub(crate) fn new(mode: &str) -> Self {
        match mode {
            "keycloak" => Authenticator::Keycloak(keycloak::Authenticator::new()),
            "in-memory" => Authenticator::InMemory(in_memory::Authenticator::new()),
            _ => panic!("Unsupported auth mode: {}", mode),
        }
    }

    pub(crate) fn get_user(&mut self, token: Token) -> Option<User> {
        match self {
            Authenticator::Keycloak(x) => x.state.get_user(token),
            Authenticator::InMemory(x) => x.state.get_user(token),
        }
    }

    pub(crate) async fn login(&mut self, username: String, password: String) -> Result<Token, String> {
        match self {
            Authenticator::Keycloak(x) => x.login(username, password).await,
            Authenticator::InMemory(x) => x.login(username, password).await,
        }
    }
}
