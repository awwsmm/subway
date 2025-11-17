use crate::newdb::tables::posts_by_id::PostsByIdTableLike;
use std::ops::DerefMut;

pub(crate) mod in_memory;
pub(crate) mod postgres;
pub(crate) mod tables;
mod table;

/// All implemented Databases are listed here.
pub(crate) enum Database {
    Postgres(postgres::Database),
    InMemory(in_memory::Database),
}

impl Database {
    pub(crate) fn posts_by_id(&mut self) -> &mut dyn PostsByIdTableLike {
        match self {
            Database::Postgres(inner) => inner.posts_by_id.deref_mut(),
            Database::InMemory(inner) => inner.posts_by_id.deref_mut(),
        }
    }
}