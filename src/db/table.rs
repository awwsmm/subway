// provides a default in_memory Table implementation
pub(crate) mod in_memory;

// Row is generic (as opposed to containing an associated type) because we might implement Row
// multiple times for the same type. For example...
//
//   impl Row<Uuid> for Post // a table with all information about all Posts
//   impl Row<Author> for Post // a table with Posts by Author, to easily find all Posts by a single Author
//   impl Row<DateTime> for Post // a table with Posts by creation time, to easily find recent Posts
//
pub(crate) trait TableRow<PrimaryKey> {
    fn primary_key(&self) -> &PrimaryKey;
}

// Table is generic for the same reason.
pub(crate) trait Table<PrimaryKey, Row> where Row: TableRow<PrimaryKey> {
    fn insert(&mut self, row: Row) -> Result<PrimaryKey, String>;

    // get must return V, not &V, because diesel::query_dsl::RunQueryDsl<Conn>::first returns an owned value
    fn get(&self, key: &PrimaryKey) -> Result<Row, String>;
}