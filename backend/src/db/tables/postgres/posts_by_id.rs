use crate::db::table::TableRow;
use crate::db::tables::posts_by_id::{PostsByIdTableLike, PostsByIdTableRow};
use diesel::dsl::insert_into;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{table, QueryDsl, SelectableHelper};
use diesel::{PgConnection, RunQueryDsl};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

table! {
    posts_by_id {
        id -> Uuid,
        title -> Text,
    }
}

#[derive(Debug)]
pub(crate) struct Impl {
    pub(crate) connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

// TODO pull this implementation out into a default trait

impl PostsByIdTableLike for Impl {
    fn insert(&mut self, row: PostsByIdTableRow) -> Result<Uuid, String> {
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

    fn get(&self, key: &Uuid) -> Result<PostsByIdTableRow, String> {
        match self.connection_pool.get() {
            Ok(mut connection) => {
                match posts_by_id::table.find(key).first::<PostsByIdTableRow>(&mut connection) {
                    Ok(post) => Ok(post),
                    Err(e) => Err(format!("Unable to find Post: {}", e)),
                }
            }
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),
        }
    }

    fn list(&self, limit: u32) -> Result<Vec<PostsByIdTableRow>, String> {
        match self.connection_pool.get() {
            Ok(mut connection) => {
                match posts_by_id::table.select(PostsByIdTableRow::as_select()).limit(limit as i64).load(&mut connection) {
                    Ok(posts) => Ok(posts),
                    Err(e) => Err(format!("Unable to get Posts: {}", e)),
                }
            },
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),

        }
    }
}