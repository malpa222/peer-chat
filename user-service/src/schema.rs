table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        username -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
