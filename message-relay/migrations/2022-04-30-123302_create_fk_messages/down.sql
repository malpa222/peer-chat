-- This file should undo anything in `up.sql`
alter table messages drop constraint messages_users_id_fk;
alter table messages drop constraint messages_chats_id_fk;