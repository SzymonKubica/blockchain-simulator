use std::{
    fs::File,
    io::{self, Read},
    str::from_utf8,
};
use sha256::digest;

use crate::model::blockchain::{Block, Header, Transaction};
use crate::hashing::hashing::Hashable;

mod hashing;
mod model;

fn main() {
    let blockchain = load_blockchain().unwrap();
    let most_recent_block = find_most_recent_block(&blockchain);

    let transactions = load_transactions().unwrap();
    let executable_transactions =
        find_executable_transactions(transactions, most_recent_block.header.timestamp + 10);

    println!(
        "{}",
        serde_json::to_string_pretty(most_recent_block).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&executable_transactions).unwrap()
    );

    // We can process up to 100 transactions in a block
    let transactions_to_process = &executable_transactions[..100];

    let block = produce_new_block(transactions_to_process.to_vec(), most_recent_block);

    println!(
        "Successfully mined the next block: {}",
        serde_json::to_string_pretty(&block).unwrap()
    );
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

fn load_transactions() -> Result<Vec<Transaction>, String> {
    let file_str_contents = read_file_contents("mempool.json").unwrap();
    let transactions: Vec<Transaction> = serde_json::from_str(&file_str_contents).unwrap();
    Ok(transactions)
}

fn find_executable_transactions(
    mut transactions: Vec<Transaction>,
    new_block_timestamp: u32,
) -> Vec<Transaction> {
    // Need to sort the transactions in the decreasing order of their fees.
    transactions
        .sort_by(|t1: &Transaction, t2: &Transaction| t2.transaction_fee.cmp(&t1.transaction_fee));

    transactions
        .into_iter()
        .filter(|t| t.lock_time > new_block_timestamp)
        .collect()
}

fn compute_transaction_hashes(transactions: Vec<Transaction>) -> Vec<String> {
    return transactions.iter().map(|t| t.hash()).collect();
}

fn compute_merkle_tree_root(transaction_hashes: Vec<String>) -> String {
    // is the comparison operator used here the string or numberical comparison?
    let null_string = "0x0000000000000000000000000000000000000000000000000000000000000000";

    let mut hashes: Vec<String> = transaction_hashes;

    while hashes.len() > 1 {
        let mut next_level_hashes: Vec<String> = vec![];
        if hashes.len() % 2 != 0 {
            hashes.push(null_string.to_string());
        }
        for i in 0..(hashes.len() / 2) {
            let a = hashes.get(2 * i);
            let b = hashes.get(2 * i + 1);

            if a > b {
                next_level_hashes.push(digest(a.unwrap().to_string() + &b.unwrap().to_string()));
            } else {
                next_level_hashes.push(digest(b.unwrap().to_string() + &a.unwrap().to_string()));
            }
        }
        hashes = next_level_hashes;
    }

    return "0x".to_owned() + hashes.get(0).unwrap();
}

fn produce_new_block(transactions: Vec<Transaction>, previous_block: &Block) -> Block {
    let transaction_hashes = compute_transaction_hashes(transactions.to_vec());

    println!("Transaction Hashes:");
    for transaction in transaction_hashes.clone() {
        println!("{}", transaction);
    }

    let merkle_root = compute_merkle_tree_root(transaction_hashes.clone());

    println!("Merkle root: {}", merkle_root.clone());

    let mut header = Header {
        difficulty: previous_block.header.difficulty,
        height: previous_block.header.height + 1,
        miner: previous_block.header.miner.clone(),
        nonce: 0,
        hash: "".to_string(),
        previous_block_header_hash: previous_block.header.hash.clone(),
        timestamp: previous_block.header.timestamp + 10,
        transactions_count: transaction_hashes.len().try_into().unwrap(),
        transactions_merkle_root: merkle_root,
    };

    let mut block_header_hash = header.hash();

    while !is_valid_block_header_hash(&block_header_hash, 5) {
        header.nonce += 1;
        println!("Trying nonce: {}", header.nonce);
        block_header_hash = header.hash();
    }

    header.hash = block_header_hash;

    Block {
        header,
        transactions,
    }
}

fn is_valid_block_header_hash(hash: &str, difficulty: usize) -> bool {
    // The hash string should have n=difficulty leading zeros
    return hash[2..(2 + difficulty)] == "0".repeat(difficulty);
}
