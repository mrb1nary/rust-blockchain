use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction{
    sender: String,
    recipient: String,
    amount: f32,
    timestamp: i64,
}

impl  Transaction {
    pub fn new(sender: String, recipient: String, amount: f32) -> Self{
        Transaction{
            sender,
            recipient,
            amount,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}