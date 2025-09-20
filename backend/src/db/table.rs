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
    fn insert(&mut self, row: Vec<Row>) -> Result<Vec<PrimaryKey>, String>;

    // get must return Row, not &Row, because diesel::query_dsl::RunQueryDsl<Conn>::first returns an owned value
    fn get(&self, key: &PrimaryKey) -> Result<Row, String>;

    fn list(&self, limit: u32) -> Result<Vec<Row>, String>;
}