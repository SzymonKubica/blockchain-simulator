use args::args::Args;

use clap::Parser;
use model::simulator::SimulatorMode;
use node::{miner::produce_blocks, validator::generate_inclusion_proof};
use views::views::show_transaction_hash;

mod args;
mod data_sourcing;
mod hashing;
mod model;
mod node;
mod views;

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
        SimulatorMode::GenerateInclusionProof => generate_inclusion_proof(args.into()),
        SimulatorMode::VerifyInclusionProof => todo!(),
        SimulatorMode::GenerateTransactions => todo!(),
    }
}
