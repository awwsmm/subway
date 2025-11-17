use crate::db::tables::posts_by_id::PostsByIdTableLike;

// defines what a 'Table' is
pub(in crate::db) mod table;

// gives a list of Tables
pub(in crate::db) mod tables;

// list the Tables we want to use here
pub(crate) struct Database {
    pub(in crate::db) posts_by_id: Box<dyn PostsByIdTableLike>,
}

impl Database {
    pub(crate) fn new() -> Self {
        Database {
            posts_by_id: Box::new(tables::posts_by_id::Impl::new()),
        }
    }
}