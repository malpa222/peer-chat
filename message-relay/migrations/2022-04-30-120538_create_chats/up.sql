-- Your SQL goes here
create table chats (
    id serial primary key,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp
);