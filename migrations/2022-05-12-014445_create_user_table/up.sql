CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    id uuid primary key default uuid_generate_v4 (),
    user_id varchar(255) unique not null,
    nickname varchar(255) default '',
    face_url varchar(255) default '',
    gender int default 0,
    phone_number varchar(32) unique,
    birth int default 0,
    email varchar(64) unique,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    app_manager_level int,
    ex varchar(1024) default '',
    attached_info varchar(1024) default '',
    is_deleted boolean default 'f'
);