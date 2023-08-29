CREATE DATABASE hellman;

create table pre_deposit_events (
    -- TODO: add primary key
    pk BIGSERIAL PRIMARY KEY,
    flattened BOOLEAN DEFAULT FALSE, 

    pre_deposit_filter JSON,
    log_meta JSON,

    -- Redundancy for convenience
    block_number BIGINT
);

create table bls_keystore (
    pk BIGSERIAL PRIMARY KEY,
    key_store JSON,
    deposit_data_pk BIGINT NULL
);

CREATE table deposit_data (
    pk BIGSERIAL PRIMARY KEY,
    pre_deposit_event_pk BIGINT,

    signature VARCHAR,
    deposit_data_root VARCHAR,
    -- Withdrawl credential could be retreived from pre_deposit_event_pk
    -- Redundancy for convenience
    withdrawal_credential VARCHAR
);

drop table pre_deposit_events;
drop table bls_keystore;

drop table deposit_data;