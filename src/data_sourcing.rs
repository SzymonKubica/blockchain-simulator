pub mod data_provider {
    use std::{
        fs::File,
        io::{self, Read},
        str::from_utf8,
    };

    use crate::model::blockchain::{Block, Transaction, InclusionProof};

    pub fn load_blockchain(source_file_name: &str) -> Result<Vec<Block>, String> {
        let file_str_contents = read_file_contents(source_file_name).unwrap();
        let blockchain: Vec<Block> = serde_json::from_str(&file_str_contents).unwrap();
        Ok(blockchain)
    }

    pub fn load_inclusion_proof(source_file_name: &str) -> Result<InclusionProof, String> {
        let file_str_contents = read_file_contents(source_file_name).unwrap();
        let proof: InclusionProof = serde_json::from_str(&file_str_contents).unwrap();
        Ok(proof)
    }

    pub fn load_transactions(file_name: &str) -> Result<Vec<Transaction>, String> {
        let file_str_contents = read_file_contents(file_name).unwrap();
        let transactions: Vec<Transaction> = serde_json::from_str(&file_str_contents).unwrap();
        Ok(transactions)
    }

    pub fn read_file_contents(file_name: &str) -> Result<String, io::Error> {
        let mut buffer = Vec::new();
        let mut file = File::open(file_name)?;
        file.read_to_end(&mut buffer)?;
        let file_contents: &str = from_utf8(&buffer).unwrap();
        Ok(file_contents.to_string())
    }
}
