use crate::db::table::in_memory::InMemoryTable;
use crate::db::table::Table;
use crate::db::tables::posts_by_id::{PostsByIdTableLike, PostsByIdTableRow};
use uuid::Uuid;

pub(crate) struct Impl {
    delegate: InMemoryTable<Uuid, PostsByIdTableRow>,
}

// We add a new() function to avoid making 'delegate' public
impl Impl {
    pub(crate) fn new() -> Self {
        Self { delegate: InMemoryTable::new() }
    }
}

impl PostsByIdTableLike for Impl {
    fn insert(&mut self, row: PostsByIdTableRow) -> Result<Uuid, String> {
        self.delegate.insert(row)
    }

    fn get(&self, key: &Uuid) -> Result<PostsByIdTableRow, String> {
        self.delegate.get(key)
    }

    fn list(&self, limit: u32) -> Result<Vec<PostsByIdTableRow>, String> {
        self.delegate.list(limit)
    }
}