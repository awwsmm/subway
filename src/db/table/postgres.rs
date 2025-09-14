use std::fmt::Debug;
use crate::db::table::{TableRow, Table};
use std::sync::Arc;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Debug)]
pub(crate) struct PostgresTable {
    connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl PostgresTable {
    pub(crate) fn new(connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Self {
        Self { connection_pool }
    }
}

impl<PrimaryKey, Row> Table<PrimaryKey, Row> for PostgresTable
where
    Row: TableRow<PrimaryKey>,
{
    fn insert(&mut self, row: Row) -> Result<PrimaryKey, String> {
        todo!()
    }

    fn get(&self, key: &PrimaryKey) -> Result<Row, String> {
        todo!()
    }
}