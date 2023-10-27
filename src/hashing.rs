pub mod hashing {
    use sha256::digest;

    use crate::model::blockchain::{Header, Transaction};
    pub trait Hashable {
        fn hash(&self) -> String;
    }

    impl Hashable for Header {
        /// Sort all the above fields in alphabetical order by their key.
        /// 2. Produce a comma-separated string containing all the values, without
        ///    any space. Numbers (height, timestamp, nonce, transaction count,
        ///    difficulty) should be encoded as decimal value without any leading
        ///    0s. Hashes (previous block header hash, transactions merkle root) and
        ///    addresses (miner) should be hex-encoded and prepended by 0x.
        /// 3. Hash the string produced in step 2 using the SHA-256 hash function.
        fn hash(&self) -> String {
            let strings = format!(
                "{},{},{},{},{},{},{},{},{}",
                &self.difficulty.to_string().as_str(),
                &self.hash.to_string().as_str(),
                &self.height.to_string().as_str(),
                &self.miner.as_str(),
                &self.nonce.to_string().as_str(),
                &self.previous_block_header_hash.as_str(),
                &self.timestamp.to_string().as_str(),
                &self.transactions_count.to_string().as_str(),
                &self.transactions_merkle_root.to_string().as_str()
            );

            let hash: String = digest(strings);

            return "0x".to_string() + &hash;
        }
    }

    impl Hashable for Transaction {
        /// A transaction hash is created by performing the following steps:
        ///
        /// 1 Ensure that transaction fields in alphabetical order by their key.
        /// 2 Produce a comma-separated string containing all the values, without any
        ///    space. Numbers (amount, lock time, transaction fee) should be encoded as
        ///    decimal value without any leading 0s. The signature and addresses
        ///    (sender, receiver) should be hex-encoded.
        /// 3 Hash the string produced in step 2 using the SHA-256 hash function
        ///    (remember to ensure that the hex string starts with 0x).
        fn hash(&self) -> String {
            let strings = format!(
                "{},{},{},{},{},{}",
                &self.amount.to_string().as_str(),
                &self.lock_time.to_string().as_str(),
                &self.receiver.as_str(),
                &self.sender.as_str(),
                &self.signature.as_str(),
                &self.transaction_fee.to_string().as_str()
            );
            let hash: String = digest(strings.to_string());

            return hash;
        }
    }
}
