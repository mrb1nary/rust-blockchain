use colored::*;
use uuid::Uuid;

use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Wallet {
    pub address: String,
    pub balance: f64,
}

impl Wallet {
    pub fn new() -> Self {
        let wallet = Wallet {
            address: Uuid::new_v4().to_string(),
            balance: 10.0,
        };

        println!(
            "💳 {} Wallet Created 🆔\n{} {}\n{} {}",
            "New".green().bold(),
            "Address:".cyan(),
            wallet.address.yellow(),
            "Initial Balance:".cyan(),
            format!("${:.2}", wallet.balance).green()
        );

        wallet
    }

    pub fn transfer(&mut self, recipient: String, amount: f64) -> Option<Transaction> {
        println!(
            "💸 {} Transfer Attempt 💰\n{} {}\n{} ${:.2}\n{} ${}",
            "Wallet".blue().bold(),
            "From:".cyan(),
            self.address.yellow(),
            "Current Balance:".cyan(),
            self.balance,
            "Transfer Amount:".cyan(),
            amount
        );

        if self.balance >= amount {
            self.balance -= amount;

            let transaction = Transaction::new(self.address.clone(), recipient, amount);

            println!(
                "✅ {} Transaction Successful 🎉\n{} ${:.2}",
                "Transfer".green().bold(),
                "Remaining Balance:".cyan(),
                self.balance
            );

            Some(transaction)
        } else {
            println!(
                "❌ {} Insufficient Funds 💔\n{} ${:.2} < ${:.2}",
                "Transfer Failed".red().bold(),
                "Balance:".cyan(),
                self.balance,
                amount
            );
            None
        }
    }

    // Optional: Add method to check balance
    pub fn get_balance(&self) -> f64 {
        println!(
            "💰 {} Current Balance: ${}",
            "Wallet".blue(),
            format!("{:.2}", self.balance).green()
        );
        self.balance
    }
}
