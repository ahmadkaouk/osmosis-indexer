CREATE TABLE IF NOT EXISTS blocks (
    id SERIAL PRIMARY KEY,
    block_id VARCHAR(255) NOT NULL,
    block_height BIGINT NOT NULL,
    block_size INTEGER,
    time VARCHAR(255),
    proposer_address VARCHAR(255) NOT NULL,
    num_txs BIGINT,
    UNIQUE (block_height)
);

CREATE TABLE IF NOT EXISTS validators (
    id SERIAL PRIMARY KEY,
    block_height BIGINT NOT NULL,
    address VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    voting_power BIGINT,
    proposer_priority BIGINT,
    UNIQUE (block_height, address)
);

CREATE TABLE IF NOT EXISTS peers (
    id SERIAL PRIMARY KEY,
    block_height BIGINT NOT NULL,
    node_id VARCHAR(255) NOT NULL,
    remote_ip VARCHAR(255),
    peer_score INTEGER,
    UNIQUE (block_height, node_id)
);