use crate::db::table::{Row, Table};
use crate::model::post::Post;
use uuid::Uuid;

#[cfg(feature = "postgres")]
use crate::model::post::posts_by_id;

#[cfg(not(feature = "postgres"))]
use crate::db::table::in_memory::InMemoryTable;

#[cfg(feature = "postgres")]
use diesel::dsl::insert_into;

#[cfg(feature = "postgres")]
use diesel::r2d2::{ConnectionManager, Pool};

#[cfg(feature = "postgres")]
use diesel::QueryDsl;

#[cfg(feature = "postgres")]
use diesel::{PgConnection, RunQueryDsl};

#[cfg(feature = "postgres")]
use std::sync::Arc;

impl Row<Uuid> for Post {
    fn primary_key(&self) -> &Uuid {
        self.id()
    }
}

#[cfg(not(feature = "postgres"))]
#[derive(Debug)]
pub(crate) struct PostsByIdTable {
    delegate: InMemoryTable<Uuid, Post>,
}

// We add a new() function to avoid making 'delegate' public
#[cfg(not(feature = "postgres"))]
impl PostsByIdTable {
    pub(crate) fn new() -> Self {
        Self { delegate: InMemoryTable::new() }
    }
}

#[cfg(not(feature = "postgres"))]
impl Table<Uuid, Post> for PostsByIdTable {
    fn insert(&mut self, row: Post) -> Result<Uuid, String> {
        self.delegate.insert(row)
    }

    fn get(&self, key: &Uuid) -> Result<Post, String> {
        self.delegate.get(key)
    }
}

#[cfg(feature = "postgres")]
#[derive(Debug)]
pub(crate) struct PostsByIdTable {
    pub(crate) connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

#[cfg(feature = "postgres")]
impl Table<Uuid, Post> for PostsByIdTable {
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