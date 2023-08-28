CREATE DATABASE hellman;

create table pre_deposit_events (
    address VARCHAR,
    block_number BIGINT,
    block_hash VARCHAR,
    transacton_hash VARCHAR,
    transaction_index BIGINT,
    log_index NUMERIC,
    sender VARCHAR,
    n NUMERIC,
    create_el_fee BOOLEAN,
    withdrawal_credential VARCHAR,
    el_fee_contract VARCHAR
);