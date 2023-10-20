pub mod miner {
    use log::{info, debug};
    use sha256::digest;

    use crate::{model::blockchain::{Transaction, Block, Header, MerkleTreeNode}, hashing::hashing::Hashable};

    pub fn compute_transaction_hashes(transactions: Vec<Transaction>) -> Vec<String> {
        transactions.iter().map(|t| t.hash()).collect()
    }

    pub fn compute_merkle_tree_root(transaction_hashes: Vec<String>) -> MerkleTreeNode {
        // is the comparison operator used here the string or numberical comparison?
        let null_string = "0x0000000000000000000000000000000000000000000000000000000000000000";

        let mut nodes: Vec<MerkleTreeNode> = transaction_hashes
            .iter()
            .map(|t| MerkleTreeNode {
                hash: t.to_string(),
                left: None,
                right: None,
            })
            .collect::<Vec<MerkleTreeNode>>();

        while nodes.len() > 1 {
            let mut next_level_nodes: Vec<MerkleTreeNode> = vec![];
            if nodes.len() % 2 != 0 {
                nodes.push(MerkleTreeNode {
                    hash: null_string.to_owned(),
                    left: None,
                    right: None,
                });
            }
            for i in 0..(nodes.len() / 2) {
                let node_a: &MerkleTreeNode = nodes.get(2 * i).unwrap();
                let node_b: &MerkleTreeNode = nodes.get(2 * i + 1).unwrap();
                let hash_a = node_a.hash.clone();
                let hash_b = node_b.hash.clone();

                let new_hash: String = if hash_a > hash_b {
                    "0x".to_string() + &digest(hash_a + &hash_b)
                } else {
                    "0x".to_string() + &digest(hash_b + &hash_a)
                };
                next_level_nodes.push(MerkleTreeNode {
                    hash: new_hash,
                    left: Some(Box::new(node_a.clone())),
                    right: Some(Box::new(node_b.clone())),
                })
            }
            nodes = next_level_nodes;
        }

        return nodes.get(0).unwrap().clone();
    }

    pub fn mine_new_block(transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        info!(
            "Producing a new block with {} transactions...",
            transactions.len()
        );

        info!("Computing transaction hashes...");
        let transaction_hashes = compute_transaction_hashes(transactions.to_vec());

        info!("Assembling the Merkle tree...");
        let merkle_root = compute_merkle_tree_root(transaction_hashes.clone());
        debug!("Assembled Merkle tree: \n{}", merkle_root.clone());
        info!("Merkle root: {}", merkle_root.hash);

        let mut header = Header {
            difficulty: previous_block.header.difficulty,
            height: previous_block.header.height + 1,
            miner: previous_block.header.miner.clone(),
            nonce: 0,
            hash: "".to_string(),
            previous_block_header_hash: previous_block.header.hash.clone(),
            timestamp: previous_block.header.timestamp + 10,
            transactions_count: transaction_hashes.len().try_into().unwrap(),
            transactions_merkle_root: merkle_root.hash,
        };

        debug!(
            "Assembled the header of the new block: \n{}",
            serde_json::to_string_pretty(&header).unwrap()
        );

        let mut block_header_hash = header.hash();

        info!("Mining the new block...");
        while !is_valid_block_header_hash(&block_header_hash, 5) {
            header.nonce += 1;
            let log_every_n_nonce = 100000;
            if header.nonce % log_every_n_nonce == 0 {
                info!("Tested nonce number: {}", header.nonce);
            }
            block_header_hash = header.hash();
        }

        info!(
            "The nonce required to make the header hash valid is: {}",
            header.nonce
        );

        header.hash = block_header_hash;

        info!(
            "Successfully mined the next block with header:\n{}",
            serde_json::to_string_pretty(&header).unwrap()
        );

        Block {
            header,
            transactions,
        }
    }

    pub fn is_valid_block_header_hash(hash: &str, difficulty: usize) -> bool {
        // The hash string should have n=difficulty leading zeros
        hash[2..(2 + difficulty)] == "0".repeat(difficulty)
    }
}
