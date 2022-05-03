-- Your SQL goes here
create table users (
    id serial primary key,
    email varchar not null,
    username varchar not null,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp
);