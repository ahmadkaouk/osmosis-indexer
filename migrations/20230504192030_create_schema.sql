CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    block_height BIGINT NOT NULL,
    tx_hash VARCHAR(64) NOT NULL UNIQUE,
    sender VARCHAR(255),
    receiver VARCHAR(255),
    amount NUMERIC(20, 8),
    fee NUMERIC(20, 8),
    timestamp TIMESTAMP
);

CREATE TABLE IF NOT EXISTS validators (
    id SERIAL PRIMARY KEY,
    block_height BIGINT NOT NULL,
    validator_address VARCHAR(255) NOT NULL,
    voting_power BIGINT,
    commission_rate NUMERIC(5, 4),
    total_delegated NUMERIC(20, 8),
    self_delegated NUMERIC(20, 8),
    UNIQUE (block_height, validator_address)
);

CREATE TABLE IF NOT EXISTS network_info (
    id SERIAL PRIMARY KEY,
    block_height BIGINT NOT NULL,
    peer_id VARCHAR(255) NOT NULL,
    peer_score INTEGER,
    UNIQUE (block_height, peer_id)
);