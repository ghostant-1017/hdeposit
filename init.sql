CREATE DATABASE hellman;

create table pre_deposit_events (
    -- TODO: add primary key
    pk SERIAL PRIMARY KEY,
    flattened BOOLEAN DEFAULT FALSE, 

    address VARCHAR,
    block_number BIGINT,
    block_hash VARCHAR,
    transaction_hash VARCHAR,
    transaction_index BIGINT,
    log_index NUMERIC,
    sender VARCHAR,
    n NUMERIC,
    create_el_fee BOOLEAN,
    withdrawal_credential VARCHAR,
    el_fee_contract VARCHAR
);

create table bls_keystore (
    pk SERIAL PRIMARY KEY,
    key_store JSON,
    deposit_data_pk BIGINT NULL,
);

CREATE table deposit_data (
    pk SERIAL PRIMARY KEY,
    pre_deposit_event_pk BIGINT,

    signature VARCHAR,
    deposit_data_root VARCHAR,
    -- Withdrawl credential could be retreived from pre_deposit_event_pk
    -- Redundancy for convenience
    withdrawal_credential VARCHAR,
)