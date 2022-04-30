-- Your SQL goes here
create table messages (
    id serial primary key,
    user_id integer not null,
    chat_id integer not null,
    created_at timestamp default current_timestamp,
    updated_at timestamp default current_timestamp
);