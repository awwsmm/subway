// provides a default in_memory Table implementation
pub(crate) mod in_memory;

// It's unlikely that we'll have two tables with the same columns (same row type) but different
// primary keys. So this can be reimplemented using an associated type, rather than a generic type
// parameter, if necessary.
pub(crate) trait TableRow<PrimaryKey> {
    fn primary_key(&self) -> &PrimaryKey;
}

// Table can be reimplemented with associated types for the same reason.
pub(crate) trait Table<PrimaryKey, Row> where Row: TableRow<PrimaryKey> {

    /// Insert one or more rows into the table.
    fn insert(&mut self, row: Vec<Row>) -> Result<Vec<PrimaryKey>, String>;

    /// Get a row from the table by its primary key.
    fn get(&self, key: &PrimaryKey) -> Result<Row, String>;

    /// List all rows from the table, up to some limit.
    fn list(&self, limit: usize) -> Result<Vec<Row>, String>;
}