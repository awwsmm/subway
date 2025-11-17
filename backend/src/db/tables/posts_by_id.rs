use crate::model::post::Post;
use crate::db::postgres::tables::posts_by_id::posts_by_id;
use crate::db::table::TableRow;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = posts_by_id)] // FIXME model should not depend on Postgres impl (diesel)
pub(crate) struct PostsByIdTableRow {
    post_id: Uuid,
    author_id: Uuid,
    title: String,
    body: String,
}

impl TableRow<Uuid> for PostsByIdTableRow {
    fn primary_key(&self) -> &Uuid {
        &self.post_id
    }
}

impl From<Post> for PostsByIdTableRow {
    fn from(value: Post) -> Self {
        Self {
            post_id: value.post_id.0.clone(),
            author_id: value.author_id.0.clone(),
            title: value.title.0.clone(),
            body: value.body.0.clone(),
        }
    }
}

pub(crate) trait PostsByIdTableLike: Sync + Send {
    fn insert(&mut self, row: Vec<PostsByIdTableRow>) -> Result<Vec<Uuid>, String>;
    fn get(&self, key: &Uuid) -> Result<PostsByIdTableRow, String>;
    fn list(&self, limit: usize) -> Result<Vec<PostsByIdTableRow>, String>;
}