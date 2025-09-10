use crate::db::tables::posts_by_id::PostsByIdTable;

pub(crate) mod tables;
pub(crate) mod table;

pub(crate) struct Database {
    pub(crate) posts_by_id: PostsByIdTable
}