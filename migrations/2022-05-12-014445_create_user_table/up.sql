CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    user_id uuid primary key default uuid_generate_v4 (),
    nickname varchar(255) not null default '',
    face_url varchar(255) not null default '',
    gender int not null default 0,
    phone_number varchar(32) not null default '',
    birth int,
    email varchar(64),
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    app_manger_level int,
    ex varchar(1024) default '',
    attached_info varchar(1024) default '',
    is_deleted boolean default 'f'
)