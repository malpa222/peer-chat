-- Your SQL goes here
alter table user_chats drop constraint user_chats_users_id_fk;
alter table user_chats drop constraint user_chats_chats_id_fk;