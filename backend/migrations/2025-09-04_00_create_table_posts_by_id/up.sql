-- 2025-09-04_00_create_table_posts_by_id/up.sql
CREATE TABLE posts_by_id (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL
);