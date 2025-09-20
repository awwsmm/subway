use crate::db::table::in_memory::InMemoryTable;
use crate::db::table::Table;
use crate::db::tables::authors_by_id::{AuthorsByIdTableLike, AuthorsByIdTableRow};
use uuid::Uuid;

pub(crate) struct Impl {
    delegate: InMemoryTable<Uuid, AuthorsByIdTableRow>,
}

// We add a new() function to avoid making 'delegate' public
impl Impl {
    pub(crate) fn new() -> Self {
        Self { delegate: InMemoryTable::new() }
    }
}

impl AuthorsByIdTableLike for Impl {
    fn insert(&mut self, rows: Vec<AuthorsByIdTableRow>) -> Result<Vec<Uuid>, String> {
        self.delegate.insert(rows)
    }

    fn get(&self, key: &Uuid) -> Result<AuthorsByIdTableRow, String> {
        self.delegate.get(key)
    }

    fn list(&self, limit: u32) -> Result<Vec<AuthorsByIdTableRow>, String> {
        self.delegate.list(limit)
    }
}