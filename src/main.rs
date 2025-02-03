mod block;
mod blockchain;
mod transaction;
mod wallet;

use blockchain::Blockchain;
use wallet::Wallet;

use colored::*;
use std::thread;
use std::time::Duration;

fn main() {
    println!("{}", "🚀 Rust Blockchain Simulation 🔗".green().bold());
    println!("{}", "===================================".blue());

    // Create Blockchain
    println!("\n🧱 {} Initializing Blockchain", "Step 1:".yellow());
    let mut blockchain = Blockchain::new();

    // Create Wallets
    println!("\n💳 {} Creating Wallets", "Step 2:".yellow());
    let mut soldaddy_wallet = Wallet::new();
    let mut bob_wallet = Wallet::new();
    let mut charlie_wallet = Wallet::new();
    let mut dave_wallet = Wallet::new();

    // Perform Transactions
    println!("\n💸 {} Performing Transactions", "Step 3:".yellow());

    // Transaction 1: SolDaddy sends to Bob
    if let Some(transaction1) = soldaddy_wallet.transfer(bob_wallet.address.clone(), 5.0) {
        blockchain.add_block(format!("SolDaddy sends 5.0 to Bob"));
    }

    // Transaction 2: Bob sends to Charlie
    if let Some(transaction2) = bob_wallet.transfer(charlie_wallet.address.clone(), 2.0) {
        blockchain.add_block(format!("Bob sends 2.0 to Charlie"));
    }

    // Transaction 3: Charlie sends to Dave
    if let Some(transaction3) = charlie_wallet.transfer(dave_wallet.address.clone(), 1.5) {
        blockchain.add_block(format!("Charlie sends 1.5 to Dave"));
    }

    // Transaction 4: Dave sends back to SolDaddy
    if let Some(transaction4) = dave_wallet.transfer(soldaddy_wallet.address.clone(), 0.5) {
        blockchain.add_block(format!("Dave sends 0.5 to SolDaddy"));
    }

    // Validate Blockchain
    println!("\n🔍 {} Blockchain Validation", "Step 4:".yellow());
    blockchain.is_valid();

    // Display Blockchain Details
    println!("Total Blocks: {}", blockchain.total_blocks());

    // Print Block Details
    for (index, block) in blockchain.chain.iter().enumerate() {
        println!("\n🧱 Block #{} Details:", index.to_string().yellow());
        println!("   Index: {}", block.index);
        println!("   Timestamp: {}", block.timestamp);
        println!("   Previous Hash: {}", block.prev_hash.blue());
        println!("   Current Hash: {}", block.hash.green());
        println!("   Data: {}", block.data.magenta());
    }

    // Simulate some processing time
    println!("\n⏳ Simulating blockchain processing...");
    thread::sleep(Duration::from_secs(2));

    println!(
        "\n🎉 {} Blockchain Simulation Complete! 🎊",
        "Success".green().bold()
    );
}
