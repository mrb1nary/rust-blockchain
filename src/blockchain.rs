use std::ops::Index;

use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis block"));

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        // let new_block = Block::new(previous_block.index+1, prev_block, data);
        let new_block = Block::new(previous_block.index + 1, previous_block.hash.clone(), data);

        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            //Check if previous hash block is correct
            if current_block.prev_hash != previous_block.hash {
                return false;
            }

            //Check if current hash block is correct
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
        }
        true
    }
}
