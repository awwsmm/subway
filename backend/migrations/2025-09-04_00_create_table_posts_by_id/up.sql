-- 2025-09-20_00_create_table_authors_by_id/up.sql
CREATE TABLE authors_by_id (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL
);