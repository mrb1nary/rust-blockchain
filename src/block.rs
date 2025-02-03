use chrono::Utc;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u32,
    pub data: String,
}

impl Block {
    pub fn new(index: u32, prev_hash: String, data: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp: timestamp as u128,
            hash: String::new(),
            prev_hash,
            nonce: 0,
            data,
        };
        block.mine();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data_to_hash = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.prev_hash, self.nonce, self.data
        );

        let mut hasher = Sha256::new();
        hasher.update(data_to_hash);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self) {
        let difficulty_level = "00000";

        loop {
            self.nonce += 1;
            let hash = self.calculate_hash();

            if hash.starts_with(difficulty_level) {
                self.hash = hash;
                break;
            }
        }
    }
}
