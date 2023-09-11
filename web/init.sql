CREATE TABLE sync_states (
    name VARCHAR UNIQUE NOT NULL,
    val BIGINT NOT NULL
)

CREATE TABLE hellman_validators (
    index BIGINT PRIMARY KEY,
    pubkey VARCHAR NOT NULL,
    withdrawal_credentials VARCHAR NOT NULL,
    amount BIGINT NOT NULL,
    data JSON
);

CREATE TABLE withdrawals (
    index BIGINT PRIMARY KEY,
    validator_index BIGINT NOT NULL,
    address VARCHAR NOT NULL,
    amount BIGINT NOT NULL
);

CREATE INDEX validators_wc_idx ON hellman_validators(withdrawal_credentials);
