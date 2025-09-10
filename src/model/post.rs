use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Post {
    id: Uuid,
    title: String,
}

// We add a new() function to avoid
//  - exposing the 'id' field to mutation
//  - users creating 'title's of unbounded length
impl Post {
    pub(crate) fn new(title: String) -> Self {

        // TODO add title validation

        Self {
            id: Uuid::new_v4(),
            title,
        }

    }

    pub(crate) fn id(&self) -> &Uuid {
        &self.id
    }
}