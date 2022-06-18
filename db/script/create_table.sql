-- updated_atのトリガー郡
-- https://zenn.dev/mpyw/articles/rdb-ids-and-timestamps-best-practices#updated_at-（update-時のデフォルト埋め）
CREATE FUNCTION refresh_updated_at_step1() RETURNS trigger AS
$$
BEGIN
    IF NEW.updated_at = OLD.updated_at THEN
        NEW.updated_at := NULL;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION refresh_updated_at_step2() RETURNS trigger AS
$$
BEGIN
    IF NEW.updated_at IS NULL THEN
        NEW.updated_at := OLD.updated_at;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION refresh_updated_at_step3() RETURNS trigger AS
$$
BEGIN
    IF NEW.updated_at IS NULL THEN
        NEW.updated_at := CURRENT_TIMESTAMP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


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

CREATE TRIGGER refresh_accounts_updated_at_step1
    BEFORE UPDATE
    ON accounts
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_accounts_updated_at_step2
    BEFORE UPDATE OF updated_at
    ON accounts
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_accounts_updated_at_step3
    BEFORE UPDATE
    ON accounts
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step3();


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

CREATE TRIGGER refresh_profiles_updated_at_step1
    BEFORE UPDATE
    ON profiles
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_profiles_updated_at_step2
    BEFORE UPDATE OF updated_at
    ON profiles
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_profiles_updated_at_step3
    BEFORE UPDATE
    ON profiles
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step3();


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

CREATE TRIGGER refresh_messages_updated_at_step1
    BEFORE UPDATE
    ON messages
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step1();
CREATE TRIGGER refresh_messages_updated_at_step2
    BEFORE UPDATE OF updated_at
    ON messages
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step2();
CREATE TRIGGER refresh_messages_updated_at_step3
    BEFORE UPDATE
    ON messages
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step3();

-- operation
create table operations
(
    revision   int           not null
        constraint operations_pk
            primary key,
    op_type    int           not null,
    source     varchar(20)   not null,
    destination       text[] not null,
    created_at TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ   NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER refresh_operations_updated_at_step1
    BEFORE UPDATE
    ON operations
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step1();

CREATE TRIGGER refresh_operations_updated_at_step2
    BEFORE UPDATE OF updated_at
    ON operations
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step2();

CREATE TRIGGER refresh_operations_updated_at_step3
    BEFORE UPDATE
    ON operations
    FOR EACH ROW
EXECUTE PROCEDURE refresh_updated_at_step3();