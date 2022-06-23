-- account
create table accounts
(
    id         varchar(20)  not null
        constraint accounts_pk
            primary key,
    email      varchar(320) not null,
    username   varchar(30),
    created_at TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create unique index accounts_email_uindex
    on accounts (email);

create unique index accounts_username_uindex
    on accounts (username);


-- profile
create table profiles
(
    id             varchar(20) not null
        constraint profiles_pk
            primary key,
    display_name   varchar(20)          default 'unknown' not null,
    status_message varchar(50),
    icon_path      varchar(30),
    created_at     TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- message
create table messages
(
    id           varchar(20) not null
        constraint messages_pk
            primary key,
    "from"       varchar(20) not null,
    "to"         varchar(20) not null,
    content_type int         not null,
    metadata     json                 default '{}' not null,
    "text"       text        not null,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- operation
create table operations
(
    revision    BIGINT      not null
        constraint operations_pk
            primary key,
    op_type     int         not null,
    source      varchar(20) not null,
    destination text[]      not null,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- friends
-- create table friends
-- (
--     id       varchar(20)         not null
--         constraint friends_pk
--             primary key,
--     user_ids TEXT[] default '{}' not null
-- );

