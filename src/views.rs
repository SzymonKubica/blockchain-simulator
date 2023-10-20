// This module provides functionality for inspecting the blockchain
pub mod views {
    use log::info;

    use crate::{
        args::args::GetTransactionHashArgs, data_sourcing::data_provider::load_blockchain,
        hashing::hashing::Hashable, model::blockchain::Block,
    };

    pub fn show_transaction_hash(args: GetTransactionHashArgs) {
        info!("Loading the blockchain from {}", args.blockchain_state);
        let blockchain = load_blockchain(&args.blockchain_state).unwrap();
        let block_number: usize = args.block_number;
        let transaction_number: usize = args.transaction_number_in_block;
        if let Some(hash) = get_transaction_hash(&blockchain, block_number, transaction_number) {
            info!(
                "Hash of the transaction {} in block {}: \n{}",
                transaction_number, block_number, hash
            );
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
}
