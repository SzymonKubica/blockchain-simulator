pub mod args {
    use clap::{arg, command, Parser};

    use crate::SimulatorMode;

    #[derive(Parser, Debug)]
    #[command(author, version, about, long_about = None)]
    pub struct Args {
        #[command(subcommand)]
        pub command: SimulatorMode,

        /// File storing the initial state of the blockchain
        #[arg(long)]
        blockchain_state: Option<String>,

        /// File storing the final and intermediate state of the blockchain
        #[arg(long)]
        blockchain_state_output: Option<String>,

        /// Name of the file storing the initial mempool
        #[arg(long)]
        mempool: Option<String>,

        /// Name of the file storing the intermediate and final mempool
        #[arg(long)]
        mempool_output: Option<String>,

        /// Number of blocks to mine
        #[arg(short, long)]
        blocks_to_mine: Option<u32>,

        // Arguments for the get-transaction-hash mode
        // Number of the block that we want to index
        #[arg(long)]
        block_number: Option<usize>,

        // Number of the transaction in that block that we want to get
        #[arg(long)]
        transaction_number_in_block: Option<usize>,
    }

    pub struct ProduceBlocksArgs {
        /// File storing the initial state of the blockchain
        pub blockchain_state: String,

        /// File storing the final and intermediate state of the blockchain
        pub blockchain_state_output: String,

        /// Name of the file storing the initial mempool
        pub mempool: String,

        /// Name of the file storing the intermediate and final mempool
        pub mempool_output: String,

        /// Number of blocks to mine
        pub blocks_to_mine: u32,
    }

    impl From<Args> for ProduceBlocksArgs {
        fn from(args: Args) -> Self {
            assert!(args.command == SimulatorMode::ProduceBlocks);
            assert!(
                args.blockchain_state.is_some(),
                "File with the initial blockchain state is required"
            );
            assert!(
                args.blockchain_state_output.is_some(),
                "Output file for blockchain state is required"
            );
            assert!(
                args.mempool.is_some(),
                "File with the mempool of transactions is required."
            );
            assert!(
                args.mempool_output.is_some(),
                "Output file with for the remaining mempool is required."
            );
            assert!(
                args.blocks_to_mine.is_some(),
                "The number of blocks to mine is required."
            );

            ProduceBlocksArgs {
                blockchain_state: args.blockchain_state.unwrap(),
                blockchain_state_output: args.blockchain_state_output.unwrap(),
                mempool: args.mempool.unwrap(),
                mempool_output: args.mempool_output.unwrap(),
                blocks_to_mine: args.blocks_to_mine.unwrap(),
            }
        }
    }

    #[derive(Debug)]
    pub struct GetTransactionHashArgs {
        /// File storing the initial state of the blockchain
        pub blockchain_state: String,
        // Arguments for the get-transaction-hash mode
        // Number of the block that we want to index
        pub block_number: usize,
        // Number of the transaction in that block that we want to get
        pub transaction_number_in_block: usize,
    }

    impl From<Args> for GetTransactionHashArgs {
        fn from(args: Args) -> Self {
            assert!(args.command == SimulatorMode::GetTransactionHash);
            assert!(
                args.blockchain_state.is_some(),
                "File with the initial blockchain state is required"
            );
            assert!(
                args.block_number.is_some(),
                "Output file for blockchain state is required"
            );
            assert!(
                args.transaction_number_in_block.is_some(),
                "Output file for blockchain state is required"
            );

            GetTransactionHashArgs {
                blockchain_state: args.blockchain_state.unwrap(),
                block_number: args.block_number.unwrap(),
                transaction_number_in_block: args.transaction_number_in_block.unwrap(),
            }
        }
    }
}
