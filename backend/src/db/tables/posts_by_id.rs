use crate::db::postgres::posts_by_id::posts_by_id;
use crate::db::table::TableRow;
use crate::model::post::Post;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = posts_by_id)]
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
            post_id: value.post_id().deref().clone(),
            author_id: value.author_id().deref().clone(),
            title: value.title().deref().clone(),
            body: value.body().deref().clone(),
        }
    }
}

pub(crate) trait PostsByIdTableLike: Sync + Send {
    fn insert(&mut self, row: Vec<PostsByIdTableRow>) -> Result<Vec<Uuid>, String>;
    fn get(&self, key: &Uuid) -> Result<PostsByIdTableRow, String>;
    fn list(&self, limit: u32) -> Result<Vec<PostsByIdTableRow>, String>;
}
