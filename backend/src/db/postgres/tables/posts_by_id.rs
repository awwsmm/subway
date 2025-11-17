use crate::db::table::TableRow;
use crate::db::tables::posts_by_id::{PostsByIdTableLike, PostsByIdTableRow};
use diesel::dsl::insert_into;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{table, Connection, QueryDsl, SelectableHelper};
use diesel::{PgConnection, RunQueryDsl};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

table! {
    posts_by_id(post_id) {
        post_id -> Uuid,
        author_id -> Uuid,
        title -> Text,
        body -> Text,
    }
}

#[derive(Debug)]
pub(in crate::db) struct Impl {
    pub(in crate::db) connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

// TODO pull this implementation out into a default trait

impl PostsByIdTableLike for Impl {
    fn insert(&mut self, rows: Vec<PostsByIdTableRow>) -> Result<Vec<Uuid>, String> {
        match self.connection_pool.get() {
            Ok(mut connection) => {
                connection.transaction::<_, _, _>(|conn| {
                    rows.into_iter().try_fold(vec![], |mut vec, row| {
                        let pk = row.primary_key().clone();
                        match insert_into(posts_by_id::table).values(row).execute(conn) {
                            Ok(_) => {
                                vec.push(pk);
                                Ok(vec)
                            },
                            Err(e) => Err(e),
                        }
                    })
                }).map_err(|e| e.to_string())
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

    fn list(&self, limit: usize) -> Result<Vec<PostsByIdTableRow>, String> {
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