mod block;
mod blockchain;
mod transaction;
mod wallet;

fn main() {
    let wallet = wallet::Wallet::new();

    println!("{:?}", wallet);
}
