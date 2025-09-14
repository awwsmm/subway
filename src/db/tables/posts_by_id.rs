use crate::db::table::in_memory::InMemoryTable;
use crate::db::table::{Row, Table};
use crate::model::post::posts_by_id;
use crate::model::post::Post;
use diesel::dsl::insert_into;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::QueryDsl;
use diesel::{PgConnection, RunQueryDsl};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

impl Row<Uuid> for Post {
    fn primary_key(&self) -> &Uuid {
        self.id()
    }
}

pub(crate) trait PostsByIdTableLike: Sync + Send + Debug {
    fn insert(&mut self, row: Post) -> Result<Uuid, String>;
    fn get(&self, key: &Uuid) -> Result<Post, String>;
}

#[derive(Debug)]
pub(crate) struct PostgresPostsByIdTable {
    pub(crate) connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl PostsByIdTableLike for PostgresPostsByIdTable {
    fn insert(&mut self, row: Post) -> Result<Uuid, String> {
        match self.connection_pool.get() {
            Ok(mut connection) => {
                let key = row.primary_key().clone();
                match insert_into(posts_by_id::table).values(row).execute(&mut connection) {
                    Ok(_) => {
                        match self.get(&key) {
                            Err(e) => Err(format!("Unable writing to DB: {}", e)),
                            Ok(post) => Ok(post.primary_key().clone()),
                        }
                    },
                    Err(e) => Err(format!("Unable to insert Post: {}", e)),
                }
            }
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),
        }
    }

    fn get(&self, key: &Uuid) -> Result<Post, String> {
        match self.connection_pool.get() {
            Ok(mut connection) => {
                match posts_by_id::table.find(key).first::<Post>(&mut connection) {
                    Ok(post) => Ok(post),
                    Err(e) => Err(format!("Unable to find User: {}", e)),
                }
            }
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct InMemoryPostsByIdTable {
    delegate: InMemoryTable<Uuid, Post>,
}

// We add a new() function to avoid making 'delegate' public
impl InMemoryPostsByIdTable {
    pub(crate) fn new() -> Self {
        Self { delegate: InMemoryTable::new() }
    }
}

impl PostsByIdTableLike for InMemoryPostsByIdTable {
    fn insert(&mut self, row: Post) -> Result<Uuid, String> {
        self.delegate.insert(row)
    }

    fn get(&self, key: &Uuid) -> Result<Post, String> {
        self.delegate.get(key)
    }
}