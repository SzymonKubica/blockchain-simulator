pub mod miner {
    use std::fs;

    use crypto_bigint::U256;
    use log::{debug, info};
    use sha256::digest;

    use crate::{
        args::args::ProduceBlocksArgs,
        data_sourcing::data_provider::{load_blockchain, load_transactions},
        hashing::hashing::Hashable,
        model::blockchain::{Block, Header, MerkleTreeNode, Transaction},
    };

    pub fn produce_blocks(args: ProduceBlocksArgs) {
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

    fn find_executable_transactions(
        mut transactions: Vec<Transaction>,
        new_block_timestamp: u32,
    ) -> Vec<Transaction> {
        // Need to sort the transactions in the decreasing order of their fees.
        transactions.sort_by(|t1: &Transaction, t2: &Transaction| {
            t2.transaction_fee.cmp(&t1.transaction_fee)
        });

        transactions
            .into_iter()
            .filter(|t| t.lock_time > new_block_timestamp)
            .collect()
    }

    pub fn compute_transaction_hashes(transactions: Vec<Transaction>) -> Vec<String> {
        transactions.iter().map(|t| t.hash()).collect()
    }

    /// Here the intermediate hashes don't have 0x00 in front of them
    pub fn construct_merkle_tree(transaction_hashes: Vec<String>) -> MerkleTreeNode {
        // is the comparison operator used here the string or numerical comparison?
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

                let hash_a_value = U256::from_be_hex(node_a
                    .hash
                    .clone()
                    .trim_start_matches("0x"));
                let hash_b_value = U256::from_be_hex(node_b
                    .hash
                    .clone()
                    .trim_start_matches("0x"));

                let new_hash: String = if hash_a_value < hash_b_value {
                    digest(hash_a + &hash_b)
                } else {
                    digest(hash_b + &hash_a)
                };
                let new_node = MerkleTreeNode {
                    hash: new_hash,
                    left: Some(Box::new(node_a.clone())),
                    right: Some(Box::new(node_b.clone())),
                };
                next_level_nodes.push(new_node)
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
        let merkle_root = construct_merkle_tree(transaction_hashes.clone());
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
            transactions_merkle_root: "0x".to_string() + &merkle_root.hash,
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

    /// The hash string should have n=difficulty leading zeros to be considered
    /// valid. It also needs to start with "0x".
    pub fn is_valid_block_header_hash(hash: &str, difficulty: usize) -> bool {
        hash[2..(2 + difficulty)] == "0".repeat(difficulty)
    }
}

pub mod validator {
    use std::{cell::RefCell, fs, rc::Rc};

    use log::info;
    use sha256::{digest, Sha256Digest};

    use crate::{
        args::args::{GenerateInclusionProofArgs, VerifyInclusionProofArgs},
        data_sourcing::data_provider::{load_blockchain, load_inclusion_proof},
        model::blockchain::{InclusionProof, MerkleTreeNode},
        node::miner::{compute_transaction_hashes, construct_merkle_tree},
    };

    pub fn generate_inclusion_proof(args: GenerateInclusionProofArgs) {
        info!("Loading the blockchain from {}", args.blockchain_state);
        let blockchain = load_blockchain(&args.blockchain_state).unwrap();

        let block = blockchain.get(args.block_number - 1).unwrap();

        let transactions = &block.transactions;

        info!("Computing transaction hashes...");
        let transaction_hashes = compute_transaction_hashes(transactions.to_vec());

        info!("Assembling the Merkle tree...");
        let merkle_root = construct_merkle_tree(transaction_hashes.clone());

        let transaction_hash_to_verify = &args.transaction_hash_to_verify;

        let Some(inclusion_proof) =
            produce_inclusion_proof(merkle_root.clone(), transaction_hash_to_verify.to_string())
        else {
            info!("Transaction not found in block, no inclusion proof generated.");
            return;
        };

        let proof = serde_json::to_string_pretty(&inclusion_proof).unwrap();
        fs::write(&args.inclusion_proof, proof.clone()).unwrap();

        info!("Generated Inclusion proof:\n{}", proof);
    }

    fn produce_inclusion_proof(
        merkle_root: MerkleTreeNode,
        transaction_hash_to_verify: String,
    ) -> Option<InclusionProof> {
        let path_to_transaction = find_path_to_transaction_in_merkle_tree(
            &merkle_root,
            &transaction_hash_to_verify,
            vec![],
        )?;

        // Path to transaction starts at the root node and then includes all
        // nodes that we have to traverse to get to that transaction

        // We need to find the transaction hashes that need to be provided for the
        // inclusion proof, those are the siblings of all transactions that are included in
        // the path.

        let mut proof: Vec<String> = vec![];

        print!(
            "{}",
            serde_json::to_string_pretty(&path_to_transaction)
                .unwrap()
                .clone()
        );
        for i in 0..path_to_transaction.len() - 1 {
            let current_parent = path_to_transaction.get(i).unwrap();
            let current_node = path_to_transaction.get(i + 1).unwrap();

            // We always need to pick the node that is different from the current
            // node (the other sibling) and extract its hash to the vector of hashes.

            if current_parent.left.as_ref().unwrap().hash == current_node.hash {
                proof.push(current_parent.right.as_ref().unwrap().hash.clone());
            } else {
                proof.push(current_parent.left.as_ref().unwrap().hash.clone());
            }
        }

        let hashes = proof.into_iter().rev().collect();

        return Some(InclusionProof {
            transaction_hash: transaction_hash_to_verify,
            merkle_root: "0x".to_string() + &merkle_root.hash,
            hashes,
        });
    }

    fn find_path_to_transaction_in_merkle_tree(
        current_node: &MerkleTreeNode,
        transaction_hash_to_verify: &str,
        path_accumulator: Vec<MerkleTreeNode>,
    ) -> Option<Vec<MerkleTreeNode>> {
        let mut new_path_accumulator = path_accumulator.clone();
        new_path_accumulator.push(current_node.clone());
        if current_node.hash == transaction_hash_to_verify {
            return Some(new_path_accumulator.to_vec());
        }

        if let Some(node) = &current_node.left {
            let maybe_found = find_path_to_transaction_in_merkle_tree(
                node,
                transaction_hash_to_verify,
                new_path_accumulator.clone(),
            );
            if maybe_found.is_some() {
                return maybe_found;
            }
        }

        if let Some(node) = &current_node.right {
            let maybe_found = find_path_to_transaction_in_merkle_tree(
                node,
                transaction_hash_to_verify,
                new_path_accumulator.clone(),
            );
            if maybe_found.is_some() {
                return maybe_found;
            }
        }

        return None;
    }

    pub fn verify_inclusion_proof(args: VerifyInclusionProofArgs) {
        info!("Loading the blockchain from {}", args.blockchain_state);
        let blockchain = load_blockchain(&args.blockchain_state).unwrap();

        info!("Loading the inclusion proof from {}", args.inclusion_proof);
        let proof: InclusionProof = load_inclusion_proof(&args.inclusion_proof).unwrap();

        let Some(block) = blockchain.get(args.block_number - 1) else {
            info!("Block not found in blockchain.");
            return;
        };

        info!("Checking of the merkle root in the inclusion proof matches the requested block");
        if block.header.transactions_merkle_root != proof.merkle_root {
            info!("Merkle root in the proof does not match the block merkle root.");
            return;
        };

        info!("Verifying the proof...");
        if let Ok(proof) = proof.verify() {
            info!("The proof is valid!");
            info!("Proof:\n{}", serde_json::to_string_pretty(&proof).unwrap());
        } else {
            info!("The proof is invalid!");
        }
    }
}
