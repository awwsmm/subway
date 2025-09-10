use crate::db::table::{Row, Table};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;

pub(crate) struct InMemoryTable<K, V> where K: Eq + Hash + Clone {
    data: HashMap<K, V>,
}

// We add a new() function to avoid making 'data' public
impl<K, V> InMemoryTable<K, V> where K: Eq + Hash + Clone {
    pub(crate) fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl<K, V> Table<K, V> for InMemoryTable<K, V> where V: Row<K>, K: Eq + Hash + Clone, V: Clone {
    async fn insert(&mut self, row: V) -> Result<K, String> {
        let key = row.primary_key().clone();
        self.data.insert(row.primary_key().clone(), row);
        Ok(key)
    }

    async fn get(&self, key: &K) -> Result<V, String> {
        match self.data.get(key) {
            None => Err("Key not found".to_string()),
            Some(value) => Ok(value.clone()),
        }
    }
}

impl<K, V> Debug for InMemoryTable<K, V> where V: Row<K>, K: Eq + Hash + Clone + Debug, V: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.data.iter()).finish()
    }
}
