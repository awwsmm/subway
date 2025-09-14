use crate::db::table::Row;
use crate::model::post::Post;
use std::fmt::Debug;
use uuid::Uuid;

impl Row<Uuid> for Post {
    fn primary_key(&self) -> &Uuid {
        self.id()
    }
}

pub(crate) trait PostsByIdTableLike: Sync + Send + Debug {
    fn insert(&mut self, row: Post) -> Result<Uuid, String>;
    fn get(&self, key: &Uuid) -> Result<Post, String>;
}
