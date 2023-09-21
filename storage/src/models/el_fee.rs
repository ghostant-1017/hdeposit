use ethers::types::{H256, H160};


pub struct ELFee {
    pub slot: u64,
    pub block_number: u64,
    pub block_hash: H256,
    pub validator_index: u64,
    pub fee_recipient: H160,
    pub amount: u64,
}


