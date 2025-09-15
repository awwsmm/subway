-- 20250904100732_create_table_posts_by_id/up.sql
CREATE TABLE posts_by_id (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL
);