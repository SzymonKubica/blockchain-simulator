use std::{
    fmt::Display,
    fs::{self, File},
    io::{self, Read},
    str::from_utf8,
};

use log::info;

use clap::{arg, command, Parser, Subcommand};

use crate::hashing::hashing::Hashable;
use crate::model::blockchain::{Block, Transaction};
use crate::node::miner::mine_new_block;

mod hashing;
mod model;
mod node;

/// Blockchain Miner Simulator

#[derive(Debug, Subcommand)]
enum SimulatorMode {
    ProduceBlocks,
    GetTransactionHash,
    GenerateInclusionProof,
    VerifyInclusionProof,
    GenerateTransactions,
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File storing the initial state of the blockchain
    #[command(subcommand)]
    command: SimulatorMode,

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
    match args.command {
        SimulatorMode::ProduceBlocks => produce_blocks(args),
        SimulatorMode::GetTransactionHash => show_transaction_hash(args),
        SimulatorMode::GenerateInclusionProof => todo!(),
        SimulatorMode::VerifyInclusionProof => todo!(),
        SimulatorMode::GenerateTransactions => todo!(),
    }
}

fn show_transaction_hash(args: Args) {
    todo!()
}

fn produce_blocks(args: Args) {

    info!("Loading the blockchain from {}", args.blockchain_state);
    let mut blockchain = load_blockchain(&args.blockchain_state).unwrap();

    info!("Loading the available mempool from {}", args.mempool);
    let transactions = load_transactions(&args.mempool).unwrap();

    let mut most_recent_block = blockchain
        .iter()
        .max_by(|b1: &&Block, b2: &&Block| b1.header.timestamp.cmp(&b2.header.timestamp))
        .unwrap();

    let mut executable_transactions =
        find_executable_transactions(transactions, most_recent_block.header.timestamp + 10);

    for _ in 0..args.blocks_to_mine {
        let new_block_transactions = executable_transactions.drain(0..100).collect();
        let block = mine_new_block(new_block_transactions, most_recent_block);
        blockchain.push(block);
        most_recent_block = blockchain.last().unwrap();
    }

    fs::write(
        &args.blockchain_state_output,
        serde_json::to_string_pretty(&blockchain).unwrap(),
    )
    .unwrap();

    fs::write(
        &args.mempool_output,
        serde_json::to_string_pretty(&executable_transactions).unwrap(),
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

fn load_blockchain(source_file_name: &str) -> Result<Vec<Block>, String> {
    let file_str_contents = read_file_contents(source_file_name).unwrap();
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

fn load_transactions(file_name: &str) -> Result<Vec<Transaction>, String> {
    let file_str_contents = read_file_contents(file_name).unwrap();
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
