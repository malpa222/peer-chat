table! {
    users (id) {
        id -> Int4,
        auth0_id -> Nullable<Varchar>,
        email -> Varchar,
        username -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
