-- Your SQL goes here
alter table user_chats add constraint user_chats_users_id_fk
        foreign key (user_id) references users (id);

alter table user_chats add constraint user_chats_chats_id_fk
        foreign key (chat_id) references chats (id)