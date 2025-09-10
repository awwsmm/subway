use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "postgres")]
use diesel::{Insertable, Queryable};

#[cfg(feature = "postgres")]
use diesel::table;

#[cfg(feature = "postgres")]
table! {
    posts_by_id {
        id -> Uuid,
        title -> Text,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "postgres", derive(Insertable, Queryable), diesel(table_name = posts_by_id))]
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