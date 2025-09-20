use crate::db::postgres::posts_by_id::posts_by_id;
use crate::db::table::TableRow;
use crate::model::post::Post;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = posts_by_id)]
pub(crate) struct PostsByIdTableRow {
    id: Uuid,
    title: String,
}

impl TableRow<Uuid> for PostsByIdTableRow {
    fn primary_key(&self) -> &Uuid {
        &self.id
    }
}

impl From<Post> for PostsByIdTableRow {
    fn from(value: Post) -> Self {
        Self {
            id: value.id,
            title: value.title,
        }
    }
}

pub(crate) trait PostsByIdTableLike: Sync + Send {
    fn insert(&mut self, row: Vec<PostsByIdTableRow>) -> Result<Vec<Uuid>, String>;
    fn get(&self, key: &Uuid) -> Result<PostsByIdTableRow, String>;
    fn list(&self, limit: u32) -> Result<Vec<PostsByIdTableRow>, String>;
}
