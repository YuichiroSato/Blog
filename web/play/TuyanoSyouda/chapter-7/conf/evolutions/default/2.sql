# Message schema
# --- !Ups
create table "messages" (
   "id" int auto_increment primary key,
   "person_id" int,
   "message" varchar(255) not null,
   "created" timestamp not null default current_timestamp
);
insert into "messages" values (default, 1, 'This is sample message,', default);
insert into "messages" values (default, 2, 'Hello!', default);
insert into "messages" values (default, 3, 'Here we go.', default);

# --- !Downs
drop table "messages"
