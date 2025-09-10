#[cfg(not(feature = "postgres"))]
pub(crate) mod in_memory;

#[cfg(feature = "postgres")]
pub(crate) mod postgres;

// Row is generic (as opposed to containing an associated type) because we might implement Row
// multiple times for the same type. For example...
//
//   impl Row<Uuid> for Post // a table with all information about all Posts
//   impl Row<Author> for Post // a table with Posts by Author, to easily find all Posts by a single Author
//   impl Row<DateTime> for Post // a table with Posts by creation time, to easily find recent Posts
//
pub(crate) trait Row<PrimaryKey> {
    fn primary_key(&self) -> &PrimaryKey;
}

// Table is generic for the same reason.
pub(crate) trait Table<K, V> where V: Row<K> {
    async fn insert(&mut self, row: V) -> Result<K, String>;
    async fn get(&self, key: &K) -> Result<V, String>;
}