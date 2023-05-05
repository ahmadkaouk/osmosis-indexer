use serde::Deserialize;
use tendermint::{block::Meta, validator};

#[derive(Deserialize, Debug)]
pub struct BlockInfo {
    /// ID of the block
    pub block_id: String,
    /// Height of the block
    pub height: i64,
    /// Block size
    pub block_size: i64,
    /// Current block time
    pub time: String,
    /// Validators hash of the block
    pub proposer_address: String,
    /// Number of transactions in the block
    pub num_txs: i64,
}

impl From<Meta> for BlockInfo {
    fn from(block: Meta) -> Self {
        BlockInfo {
            block_id: block.block_id.to_string(),
            height: block.header.height.into(),
            block_size: block.block_size,
            time: block.header.time.to_string(),
            proposer_address: block.header.proposer_address.to_string(),
            num_txs: block.num_txs,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ValidatorInfo {
    /// Validator account address
    pub address: String,

    pub block_height: i64,

    /// Validator public key
    pub pub_key: String,

    /// Validator voting power
    pub power: u64,

    /// Validator name
    pub name: Option<String>,

    /// Validator proposer priority
    pub proposer_priority: i64,
}

impl ValidatorInfo {
    pub fn from_info(validator: validator::Info, block_height: i64) -> Self {
        ValidatorInfo {
            address: validator.address.to_string(),
            pub_key: validator.pub_key.to_hex(),
            power: validator.power.value(),
            block_height,
            name: validator.name,
            proposer_priority: validator.proposer_priority.value(),
        }
    }
}
