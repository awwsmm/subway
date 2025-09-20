use crate::db::table::TableRow;
use crate::db::tables::authors_by_id::{AuthorsByIdTableLike, AuthorsByIdTableRow};
use diesel::dsl::insert_into;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{table, Connection, QueryDsl, SelectableHelper};
use diesel::{PgConnection, RunQueryDsl};
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

table! {
    authors_by_id {
        id -> Uuid,
        name -> Text,
    }
}

#[derive(Debug)]
pub(crate) struct Impl {
    pub(crate) connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

// TODO pull this implementation out into a default trait

impl AuthorsByIdTableLike for Impl {
    fn insert(&mut self, rows: Vec<AuthorsByIdTableRow>) -> Result<Vec<Uuid>, String> {
        match self.connection_pool.get() {
            Ok(mut connection) => {
                connection.transaction::<_, _, _>(|conn| {
                    rows.into_iter().try_fold(vec![], |mut vec, row| {
                        let pk = row.primary_key().clone();
                        match insert_into(authors_by_id::table).values(row).execute(conn) {
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

    fn get(&self, key: &Uuid) -> Result<AuthorsByIdTableRow, String> {
        match self.connection_pool.get() {
            Ok(mut connection) => {
                match authors_by_id::table.find(key).first::<AuthorsByIdTableRow>(&mut connection) {
                    Ok(author) => Ok(author),
                    Err(e) => Err(format!("Unable to find Author: {}", e)),
                }
            }
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),
        }
    }

    fn list(&self, limit: u32) -> Result<Vec<AuthorsByIdTableRow>, String> {
        match self.connection_pool.get() {
            Ok(mut connection) => {
                match authors_by_id::table.select(AuthorsByIdTableRow::as_select()).limit(limit as i64).load(&mut connection) {
                    Ok(authors) => Ok(authors),
                    Err(e) => Err(format!("Unable to get Authors: {}", e)),
                }
            },
            Err(e) => Err(format!("Unable to connect to DB: {}", e)),

        }
    }
}