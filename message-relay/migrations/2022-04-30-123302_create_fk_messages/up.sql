-- Your SQL goes here
alter table messages add constraint messages_users_id_fk
        foreign key (user_id) references users (id);

alter table messages add constraint messages_chats_id_fk
        foreign key (chat_id) references chats (id)