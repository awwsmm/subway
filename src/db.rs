#[cfg(feature = "postgres")]
use diesel::{table, Insertable, Queryable, Selectable};
use diesel::Identifiable;
use serde::Serialize;

#[cfg(feature = "postgres")]
mod postgres;

#[cfg(feature = "postgres")]
table! {
    users {
        id -> Integer,
        name -> Text,
    }
}

#[cfg(not(feature = "postgres"))]
mod in_memory;

#[derive(Serialize)]
#[cfg_attr(feature = "postgres", derive(Queryable, Selectable, Insertable, Identifiable))]
#[cfg_attr(feature = "postgres", diesel(table_name = users, primary_key(id)))]
#[cfg_attr(not(feature = "postgres"), derive(Clone))]
pub(crate) struct User {
    id: i32,
    name: String,
}

pub(crate) struct Database;

// implemented by in_memory, postgres
pub(crate) trait DatabaseLike {
    async fn connect() -> Result<(), String>;

    async fn add_user(&mut self, id: i32, name: String) -> Result<(), String>;

    async fn get_user(&self, id: i32) -> Result<User, String>;
}