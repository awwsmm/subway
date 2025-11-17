use crate::db::table::{Table, TableRow};
use std::collections::HashMap;
use std::hash::Hash;

pub(in crate::db) struct InMemoryTable<PrimaryKey, Row> {
    data: HashMap<PrimaryKey, Row>,
}

// We add a new() function to avoid making 'data' visible
impl<PrimaryKey, Row> InMemoryTable<PrimaryKey, Row> {
    pub(in crate::db) fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl<PrimaryKey, Row> Table<PrimaryKey, Row> for InMemoryTable<PrimaryKey, Row>
where
    PrimaryKey: Eq + Hash, // required by HashMap
    PrimaryKey: Clone, // required for insert() to take ownership of K
    Row: TableRow<PrimaryKey>,
    Row: Clone, // required to turn &V into V after calling .get()
{
    fn insert(&mut self, rows: Vec<Row>) -> Result<Vec<PrimaryKey>, String> {
        rows.into_iter().try_fold(vec![], |mut vec, row| {
            let pk = row.primary_key().clone();
            self.data.insert(pk.clone(), row);
            vec.push(pk);
            Ok(vec)
        })
    }

    fn get(&self, key: &PrimaryKey) -> Result<Row, String> {
        match self.data.get(key) {
            None => Err("Key not found".to_string()),
            Some(value) => Ok(value.clone()),
        }
    }

    fn list(&self, limit: usize) -> Result<Vec<Row>, String> {
        Ok(self.data.values().take(limit).cloned().collect())
    }
}
