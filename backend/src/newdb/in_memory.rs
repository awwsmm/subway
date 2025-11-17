use crate::newdb::tables::posts_by_id::PostsByIdTableLike;

// defines what a 'Table' is
pub(crate) mod table;

// gives a list of Tables
pub(crate) mod tables;

// list the Tables we want to use here
pub(crate) struct Database {
    pub(crate) posts_by_id: Box<dyn PostsByIdTableLike>,
}

impl Database {
    pub(crate) fn new() -> Self {
        Database {
            posts_by_id: Box::new(tables::posts_by_id::Impl::new()),
        }
    }
}