use args::args::{Args, GetTransactionHashArgs};

use clap::Parser;
use log::info;
use model::simulator::SimulatorMode;
use node::miner::produce_blocks;

use crate::data_sourcing::data_provider::load_blockchain;
use crate::hashing::hashing::Hashable;
use crate::model::blockchain::{Block, Transaction};

mod args;
mod data_sourcing;
mod hashing;
mod model;
mod node;

/// Blockchain Miner Simulator

fn main() {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let args = Args::parse();
    match args.command {
        SimulatorMode::ProduceBlocks => produce_blocks(args.into()),
        SimulatorMode::GetTransactionHash => show_transaction_hash(args.into()),
        SimulatorMode::GenerateInclusionProof => todo!(),
        SimulatorMode::VerifyInclusionProof => todo!(),
        SimulatorMode::GenerateTransactions => todo!(),
    }
}

fn show_transaction_hash(args: GetTransactionHashArgs) {
    info!("Loading the blockchain from {}", args.blockchain_state);
    let blockchain = load_blockchain(&args.blockchain_state).unwrap();
    let block_number: usize = args.block_number;
    let transaction_number: usize = args.transaction_number_in_block;
    if let Some(hash) = get_transaction_hash(&blockchain, block_number, transaction_number) {
        info!("Hash of the transaction {} in block {}: \n{}", transaction_number, block_number, hash);
    }
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
