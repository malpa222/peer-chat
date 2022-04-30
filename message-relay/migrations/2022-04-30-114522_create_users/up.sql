-- Your SQL goes here
create table users (
    id serial primary key,
    email varchar not null unique,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp
);