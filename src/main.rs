use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{self, Read},
    str::from_utf8,
};

#[derive(Serialize, Deserialize, Debug)]
struct Header {
    difficulty: u32,
    height: u32,
    miner: String,
    nonce: u32,
    hash: String,
    previous_block_header_hash: String,
    timestamp: u32,
    transactions_count: u32,
    transactions_merkle_root: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    amount: u64,
    lock_time: u32,
    receiver: String,
    sender: String,
    signature: String,
    transaction_fee: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    header: Header,
    transactions: Vec<Transaction>,
}

fn main() {
    let blockchain = load_blockchain().unwrap();
    let most_recent_block = find_most_recent_block(&blockchain);
    print!("{}", serde_json::to_string_pretty(most_recent_block).unwrap());
}

fn find_most_recent_block(blockchain: &Vec<Block>) -> &Block {
    blockchain
        .iter()
        .max_by(|b1: &&Block, b2: &&Block| b1.header.timestamp.cmp(&b2.header.timestamp))
        .unwrap()
}

fn load_blockchain() -> Result<Vec<Block>, String> {
    let file_str_contents = read_file_contents("blockchain.json").unwrap();
    let blockchain: Vec<Block> = serde_json::from_str(&file_str_contents).unwrap();
    Ok(blockchain)
}

fn read_file_contents(file_name: &str) -> Result<String, io::Error> {
    let mut buffer = Vec::new();
    let mut file = File::open(file_name)?;
    file.read_to_end(&mut buffer)?;
    let file_contents: &str = from_utf8(&buffer).unwrap();
    Ok(file_contents.to_string())
}
