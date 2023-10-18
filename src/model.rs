pub mod blockchain {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Header {
        pub difficulty: u32,
        pub height: u32,
        pub miner: String,
        pub nonce: u32,
        pub hash: String,
        pub previous_block_header_hash: String,
        pub timestamp: u32,
        pub transactions_count: u32,
        pub transactions_merkle_root: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Transaction {
        pub amount: u64,
        pub lock_time: u32,
        pub receiver: String,
        pub sender: String,
        pub signature: String,
        pub transaction_fee: u64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Block {
        pub header: Header,
        pub transactions: Vec<Transaction>,
    }
}
