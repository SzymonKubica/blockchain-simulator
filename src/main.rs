use serde::{Deserialize, Serialize};
use sha256::digest;
use std::{
    fs::File,
    io::{self, Read},
    str::from_utf8,
};

trait Hashable {
    fn hash(&self) -> String;
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    amount: u64,
    lock_time: u32,
    receiver: String,
    sender: String,
    signature: String,
    transaction_fee: u64,
}

impl Hashable for Transaction {
    /// A transaction hash is created by performing the following steps:
    ///
    /// 1 Ensure that transaction fields in alphabetical order by their key.
    /// 2 Produce a comma-separated string containing all the values, without any
    ///    space. Numbers (amount, lock time, transaction fee) should be encoded as
    ///    decimal value without any leading 0s. The signature and addresses
    ///    (sender, receiver) should be hex-encoded.
    /// 3 Hash the string produced in step 2 using the SHA-256 hash function
    ///    (remember to ensure that the hex string starts with 0x).
    fn hash(&self) -> String {
        let strings = format!("{},{},{},{},{},{}",
            &self.amount.to_string().as_str(),
            &self.lock_time.to_string().as_str(),
            &self.receiver.as_str(),
            &self.sender.as_str(),
            &self.signature.as_str(),
            &self.transaction_fee.to_string().as_str());

        let hash: String = digest(strings);

        return "0x".to_string() + &hash;
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    header: Header,
    transactions: Vec<Transaction>,
}

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

    println!("Transaction Hashes:");
    for transaction in compute_transaction_hashes(transactions_to_process.to_vec()) {
        println!("{}", transaction);
    }

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
    return transactions.into_iter().map(|t| t.hash()).collect();
}

fn produce_new_block(transactions: Vec<Transaction>) {}
