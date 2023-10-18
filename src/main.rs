use std::{
    fs::{self, File},
    io::{self, Read},
    str::from_utf8,
};

use clap::{Arg, Parser, command, arg};

use crate::hashing::hashing::Hashable;
use crate::model::blockchain::{Block, Transaction};
use crate::node::miner::mine_new_block;

mod hashing;
mod model;
mod node;

/// Blockchain Miner Simulator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File storing the initial state of the blockchain
    #[arg(long)]
    blockchain_state: String,

    /// File storing the final and intermediate state of the blockchain
    #[arg(long)]
    blockchain_state_output: String,

    /// Name of the file storing the initial mempool
    #[arg(long)]
    mempool: String,

    /// Name of the file storing the intermediate and final mempool
    #[arg(long)]
    mempool_output: String,

    /// Number of blocks to mine
    #[arg(short, long)]
    blocks_to_mine: u32,

}

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let args = Args::parse();

    let mut blockchain = load_blockchain().unwrap();
    let most_recent_block = find_most_recent_block(&blockchain);

    let transactions = load_transactions().unwrap();
    let executable_transactions =
        find_executable_transactions(transactions, most_recent_block.header.timestamp + 10);

    // We can process up to 100 transactions in a block
    let transactions_to_process = (&executable_transactions[..100]).to_vec();

    let block = mine_new_block(transactions_to_process, most_recent_block);
    let block2 = mine_new_block((&executable_transactions[100..200]).to_vec(), &block);

    blockchain.push(block);
    blockchain.push(block2);
    fs::write(
        "new-blockchain.js",
        serde_json::to_string_pretty(&blockchain).unwrap(),
    )
    .unwrap();
}

fn get_transaction_hash(
    blockchain: &Vec<Block>,
    block_number: usize,
    transaction_number: usize,
) -> Option<String> {
    let block = blockchain.get(block_number - 1)?;
    let transaction = block.transactions.get(transaction_number - 1)?;
    Some(transaction.hash().to_owned())
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
