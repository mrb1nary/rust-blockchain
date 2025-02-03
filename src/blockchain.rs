use colored::*;

use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        println!("🚀 {} Initializing Blockchain 🔗", "Creating".green().bold());
        
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis block"));

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();
        
        println!(
            "📦 {} Block #{} with data: {}",
            "Preparing to add".blue(),
            (previous_block.index + 1).to_string().yellow(),
            data.magenta()
        );

        let new_block = Block::new(previous_block.index + 1, previous_block.hash.clone(), data);

        self.chain.push(new_block);
        
        println!(
            "✅ {} Block #{} successfully added",
            "Blockchain".green(),
            (self.chain.len() - 1).to_string().yellow()
        );
    }

    pub fn is_valid(&self) -> bool {
        println!("🔍 {} Blockchain Validation", "Performing".cyan());
        
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            //Check if previous hash block is correct
            if current_block.prev_hash != previous_block.hash {
                println!("❌ {} Invalid blockchain: Incorrect previous hash", "Error".red());
                return false;
            }

            //Check if current hash block is correct
            if current_block.hash != current_block.calculate_hash() {
                println!("❌ {} Invalid blockchain: Incorrect block hash", "Error".red());
                return false;
            }
        }
        
        println!("✅ {} Blockchain is valid", "Success".green());
        true
    }

    pub fn total_blocks(&self) -> usize {
        let total = self.chain.len();
        println!(
            "📊 {} Total Blocks: {}",
            "Blockchain Status".blue(),
            total.to_string().yellow()
        );
        total
    }
}
