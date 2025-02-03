mod block;
mod blockchain;
mod transaction;


fn main() {
    let block = block::Block::new(0, String::from("0"), String::from("Genesis block"));
    println!("{:?}", block);

    let mut blockchain = blockchain::Blockchain::new();
    blockchain.add_block(String::from("First block"));
    blockchain.add_block(String::from("Second block"));

    println!("{:?}", blockchain);
    println!("Is blockchain valid? {}", blockchain.is_valid());

}
