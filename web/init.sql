CREATE TABLE sync_states (
    name VARCHAR UNIQUE NOT NULL,
    val BIGINT NOT NULL
)

CREATE TABLE hellman_validators (
    index BIGINT PRIMARY KEY,
    withdrawal_credentials VARCHAR NOT NULL,
    data JSON
);

CREATE INDEX validators_wc_idx ON hellman_validators;
