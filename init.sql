CREATE DATABASE hellman;

create table pre_deposit_events (
    -- TODO: add primary key
    pk SERIAL PRIMARY KEY,
    processed_n NUMERIC DEFAULT 0, 

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

create table bls_addresses (
    pk SERIAL PRIMARY KEY,
    key_store JSON,
    event_pk BIGINT NULL
);