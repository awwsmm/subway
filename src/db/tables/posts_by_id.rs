use crate::db::table::{Row, Table};
use crate::model::post::Post;
use uuid::Uuid;

#[cfg(not(feature = "postgres"))]
use crate::db::table::in_memory::InMemoryTable;

#[cfg(not(feature = "postgres"))]
#[derive(Debug)]
pub(crate) struct PostsByIdTable {
    delegate: InMemoryTable<Uuid, Post>,
}

// We add a new() function to avoid making 'delegate' public
impl PostsByIdTable {
    pub(crate) fn new() -> Self {
        Self { delegate: InMemoryTable::new() }
    }
}

impl Row<Uuid> for Post {
    fn primary_key(&self) -> &Uuid {
        self.id()
    }
}

#[cfg(not(feature = "postgres"))]
impl Table<Uuid, Post> for PostsByIdTable {
    async fn insert(&mut self, row: Post) -> Uuid {
        self.delegate.insert(row).await
    }

    async fn get<'a>(&'a self, key: &Uuid) -> Option<&'a Post> where Post: 'a {
        self.delegate.get(key).await
    }
}

#[cfg(feature = "postgres")]
impl Table<Uuid, Post> for PostsByIdTable {
    async fn insert(&mut self, row: Post) -> Result<(), String> {
        todo!()
    }

    async fn get(&self, key: Uuid) -> Result<&Post, String> {
        todo!()
    }
}