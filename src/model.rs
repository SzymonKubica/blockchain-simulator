pub mod blockchain {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};
    use sha256::digest;

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

    #[derive(Clone, Debug, Serialize)]
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

    /// The inclusion proof is used to verify that a transaction is indeed included
    /// in a block. The transaction_hash is the hash of the transaction for which we
    /// want to prove that it is included in the block. The merkle root is the
    /// hash of the whole merkle tree that is included in the hearder of the block
    /// where the said transaction is supposed to be located. Finally hashes is the
    /// list of the intermeidate hashes (one for each level of the tree that
    /// are needed to verify that the transaction belongs to that block. The first
    /// hash in the list is the direct sibling of the transaction to verify. When
    /// verifying the inclusion, we hash the two together to get the hash one level
    /// above, then the next element in the list is the hash that needs to be hashed
    /// with whatever we got in the first step. We repeat the process until the
    /// end of the list and whatever we get should equal the merkle root.
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct InclusionProof {
        pub transaction_hash: String,
        pub merkle_root: String,
        pub hashes: Vec<String>,
    }

    impl InclusionProof {
        pub fn verify(&self) -> Result<InclusionProof, String> {
            let hashes = &self.hashes;
            let mut current_hash = self.transaction_hash.clone();
            for i in 0..hashes.len() {
                let hash_a = current_hash;
                let hash_b = hashes[i].to_string();

                // The order of concatenation depends on the comparison of the
                // strings
                current_hash = if hash_a < hash_b {
                    digest(hash_a + &hash_b)
                } else {
                    digest(hash_b + &hash_a)
                };
            }
            // At this point current hash should be equal to the merkle root.
            // we need to format the current_hash with 0x accordingly.
            current_hash = "0x".to_string() + &current_hash;
            if current_hash == self.merkle_root {
                Ok(self.clone())
            } else {
                Err("Inclusion proof verification failed".to_string())
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
