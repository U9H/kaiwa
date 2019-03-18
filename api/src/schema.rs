table! {
    pages (id) {
        id -> Int4,
        site_id -> Int4,
        location -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    sites (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    threads (id) {
        id -> Int4,
        page_id -> Int4,
        thread_id -> Nullable<Int4>,
        body -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(pages -> sites (site_id));
joinable!(sites -> users (user_id));
joinable!(threads -> pages (page_id));

allow_tables_to_appear_in_same_query!(
    pages,
    sites,
    threads,
    users,
);
