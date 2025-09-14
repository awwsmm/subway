use crate::db::table::in_memory::InMemoryTable;
use crate::db::table::Table;
use crate::db::tables::posts_by_id::PostsByIdTableLike;
use crate::model::post::Post;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct Impl {
    delegate: InMemoryTable<Uuid, Post>,
}

// We add a new() function to avoid making 'delegate' public
impl Impl {
    pub(crate) fn new() -> Self {
        Self { delegate: InMemoryTable::new() }
    }
}

impl PostsByIdTableLike for Impl {
    fn insert(&mut self, row: Post) -> Result<Uuid, String> {
        self.delegate.insert(row)
    }

    fn get(&self, key: &Uuid) -> Result<Post, String> {
        self.delegate.get(key)
    }
}