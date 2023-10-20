pub mod blockchain {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
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

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Block {
        pub header: Header,
        pub transactions: Vec<Transaction>,
    }

    #[derive(Clone)]
    pub struct MerkleTreeNode {
        pub hash: String,
        pub left: Option<Box<MerkleTreeNode>>,
        pub right: Option<Box<MerkleTreeNode>>,
    }

    impl Display for MerkleTreeNode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.recursive_fmt(f, 0)
        }
    }

    impl MerkleTreeNode {
        fn recursive_fmt(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
            let indentation = "    ".repeat(depth);
            match (&self.left, &self.right) {
                (None, None) => writeln!(f, "{}Leaf: {}", indentation, self.hash),
                (None, Some(node)) => {
                    writeln!(f, "{}Node: {}", indentation, self.hash)?;
                    node.recursive_fmt(f, depth + 1)
                }
                (Some(node), None) => {
                    writeln!(f, "{}Node: {}", indentation, self.hash)?;
                    node.recursive_fmt(f, depth + 1)
                }
                (Some(node1), Some(node2)) => {
                    writeln!(f, "{}Node: {}", indentation, self.hash)?;
                    node1.recursive_fmt(f, depth + 1)?;
                    node2.recursive_fmt(f, depth + 1)
                }
            }
        }
    }
}

pub mod simulator {
    use clap::Subcommand;

    #[derive(Debug, Subcommand, PartialEq, Eq)]
    pub enum SimulatorMode {
        ProduceBlocks,
        GetTransactionHash,
        GenerateInclusionProof,
        VerifyInclusionProof,
        GenerateTransactions,
    }
}
