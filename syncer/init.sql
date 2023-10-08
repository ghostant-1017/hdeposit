CREATE TABLE protocol_reward(
    id BIGSERIAL PRIMARY KEY,
    epoch BIGINT NOT NULL,
    validator_index BIGINT NOT NULL,
    start_balance BIGINT NOT NULl,
    closing_balance BIGINT NOT NULL,
    withdrawal_amount BIGINT NOT NULL,
    reward_amount BIGINT NOT NULL
);
CREATE INDEX protocol_reward_epoch_idx ON protocol_reward(epoch);
CREATE INDEX protocol_reward_vidx_idx ON protocol_reward(validator_index);

CREATE TABLE execution_reward(
    id BIGSERIAL PRIMARY KEY,
    slot BIGINT NOT NULL,
    block_number BIGINT NOT NULL,
    block_hash VARCHAR NOT NULL,
    validator_index BIGINT NOT NULL,
    fee_recipient VARCHAR NOT NULL,
    amount BIGINT NOT NULL
);

CREATE INDEX execution_reward_slot_idx ON execution_reward(slot);
CREATE INDEX execution_reward_recipient_idx ON execution_reward(fee_recipient);
CREATE INDEX execution_reward_validator_id_idx on execution_reward(validator_index);