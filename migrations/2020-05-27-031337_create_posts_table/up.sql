-- Your SQL goes here

CREATE TABLE posts (
    num int CHECK (num >= 1),
    thread_id bigint REFERENCES threads(id),
    name text NOT NULL,
    PRIMARY KEY (num, thread_id)
);