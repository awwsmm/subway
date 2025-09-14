use crate::db::table::Row;
use crate::db::tables::posts_by_id::PostsByIdTableLike;
use crate::model::post::posts_by_id;
use crate::model::post::Post;
use diesel::dsl::insert_into;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::QueryDsl;
use diesel::{PgConnection, RunQueryDsl};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct Impl {
    pub(crate) connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl PostsByIdTableLike for Impl {
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