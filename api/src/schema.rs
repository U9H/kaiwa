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
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(sites -> users (user_id));

allow_tables_to_appear_in_same_query!(
    sites,
    users,
);
