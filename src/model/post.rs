use serde::{Deserialize, Serialize};

// FIXME make id field a Uuid when Postgres is introduced
#[derive(Serialize, Deserialize)]
pub(crate) struct Post {
    pub(crate) id: String,
    pub(crate) title: String,
}