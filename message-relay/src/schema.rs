table! {
    chats (id) {
        id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        chat_id -> Int4,
        content -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    user_chats (id) {
        id -> Int4,
        user_id -> Int4,
        chat_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(messages -> chats (chat_id));
joinable!(messages -> users (user_id));
joinable!(user_chats -> chats (chat_id));
joinable!(user_chats -> users (user_id));

allow_tables_to_appear_in_same_query!(
    chats,
    messages,
    user_chats,
    users,
);
