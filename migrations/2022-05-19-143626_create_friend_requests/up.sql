-- Your SQL goes here
create table friend_requests (
    from_user_id varchar(64) not null,
    to_user_id varchar(64) not null,
    handle_result int default 0,
    req_msg varchar(255) default '',
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    handler_user_id varchar(64) default '',
    handle_msg varchar(255) default '',
    handle_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    ex varchar(1024) default '',
    is_deleted boolean default 'f',
    primary key (from_user_id, to_user_id)
);