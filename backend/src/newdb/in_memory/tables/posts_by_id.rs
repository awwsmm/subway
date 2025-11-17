use crate::newdb::in_memory::table::InMemoryTable;
use crate::newdb::table::Table;
use crate::newdb::tables::posts_by_id::{PostsByIdTableLike, PostsByIdTableRow};
use uuid::Uuid;

pub(in crate::newdb) struct Impl {
    delegate: InMemoryTable<Uuid, PostsByIdTableRow>,
}

// We add a new() function to avoid making 'delegate' public
impl Impl {
    pub(in crate::newdb) fn new() -> Self {
        Self { delegate: InMemoryTable::new() }
    }
}

impl PostsByIdTableLike for Impl {
    fn insert(&mut self, rows: Vec<PostsByIdTableRow>) -> Result<Vec<Uuid>, String> {
        self.delegate.insert(rows)
    }

    fn get(&self, key: &Uuid) -> Result<PostsByIdTableRow, String> {
        self.delegate.get(key)
    }

    fn list(&self, limit: usize) -> Result<Vec<PostsByIdTableRow>, String> {
        self.delegate.list(limit)
    }
}