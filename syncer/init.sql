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
