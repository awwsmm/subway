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

impl<K, V> Table<K, V> for InMemoryTable<K, V> where V: Row<K>, K: Eq + Hash + Clone {
    async fn insert(&mut self, row: V) -> K {
        let key = row.primary_key().clone();
        self.data.insert(row.primary_key().clone(), row);
        key
    }

    async fn get<'a>(&'a self, key: &K) -> Option<&'a V> where V: 'a{
        self.data.get(key)
    }
}

impl<K, V> Debug for InMemoryTable<K, V> where V: Row<K>, K: Eq + Hash + Clone + Debug, V: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.data.iter()).finish()
    }
}
