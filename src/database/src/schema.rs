table! {
    boards (id) {
        id -> Varchar,
        title -> Text,
    }
}

table! {
    posts (num, thread_id) {
        num -> Int4,
        thread_id -> Int8,
        name -> Text,
    }
}

table! {
    threads (id) {
        id -> Int8,
        board -> Varchar,
        title -> Text,
        posts_count -> Int4,
    }
}

joinable!(posts -> threads (thread_id));
joinable!(threads -> boards (board));

allow_tables_to_appear_in_same_query!(
    boards,
    posts,
    threads,
);
