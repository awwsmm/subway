use crate::db::postgres::authors_by_id::authors_by_id;
use crate::db::table::TableRow;
use crate::model::author::Author;
use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Insertable, Queryable, Selectable)]
#[diesel(table_name = authors_by_id)]
pub(crate) struct AuthorsByIdTableRow {
    id: Uuid,
    name: String,
}

impl TableRow<Uuid> for AuthorsByIdTableRow {
    fn primary_key(&self) -> &Uuid {
        &self.id
    }
}

impl From<Author> for AuthorsByIdTableRow {
    fn from(value: Author) -> Self {
        Self {
            id: value.id(),
            name: value.name().to_string(),
        }
    }
}

pub(crate) trait AuthorsByIdTableLike: Sync + Send {
    fn insert(&mut self, row: Vec<AuthorsByIdTableRow>) -> Result<Vec<Uuid>, String>;
    fn get(&self, key: &Uuid) -> Result<AuthorsByIdTableRow, String>;
    fn list(&self, limit: u32) -> Result<Vec<AuthorsByIdTableRow>, String>;
}
