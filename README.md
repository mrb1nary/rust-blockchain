# 🚀 Rust Blockchain

A simple blockchain implementation in Rust featuring wallets, transactions, and a Proof-of-Stake (PoS) consensus mechanism.

## 🌟 Features

- **🔗 Blockchain Core**: A functional blockchain with blocks and transactions.
- **💰 Wallet System**: Generate and manage wallets with cryptographic key pairs.
- **💳 Transactions**: Secure peer-to-peer transactions between wallets.
- **⚡ Proof-of-Stake (PoS) Consensus**: A lightweight and efficient consensus mechanism.

## 🛠 Installation

Make sure you have Rust installed. If not, install it from [Rustup](https://rustup.rs/).

```sh
# Clone the repository
git clone https://github.com/yourusername/rust-blockchain.git
cd rust-blockchain

# Build the project
cargo build --release

# Run the blockchain node
cargo run
```

## 📊 Blockchain Architecture

```mermaid
classDiagram
    class Block {
        +index: u32
        +timestamp: u128
        +hash: String
        +prev_hash: String
        +nonce: u32
        +data: String
        +new(index: u32, prev_hash: String, data: String) Block
        +calculate_hash() String
        +mine() void
    }

    class Transaction {
        +id: String
        +sender: String
        +recipient: String
        +amount: f64
        +timestamp: i64
        +new(sender: String, recipient: String, amount: f64) Transaction
    }

    class Wallet {
        +address: String
        +balance: f64
        +new() Wallet
        +transfer(recipient: String, amount: f64) Option<Transaction>
    }

    class Blockchain {
        +chain: Vec<Block>
        +new() Blockchain
        +add_block(data: String) void
        +is_valid() bool
        +total_blocks() usize
    }

    Wallet --> Transaction: creates
    Blockchain --> Block: contains
    Block --> Transaction: stores data
    Transaction --> Wallet: links sender/recipient
```

