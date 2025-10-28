-- 2025-09-04_00_create_table_posts_by_id/up.sql
CREATE TABLE posts_by_id (
    post_id UUID PRIMARY KEY,
    author_id UUID NOT NULL,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL
);