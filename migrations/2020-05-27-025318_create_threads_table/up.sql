-- Your SQL goes here

CREATE TABLE threads (
    id bigint DEFAULT extract(epoch from now() at time zone 'utc') PRIMARY KEY,
    board VARCHAR(20) NOT NULL REFERENCES boards(id),
    title TEXT NOT NULL,
    posts_count int DEFAULT 0 NOT NULL
);
