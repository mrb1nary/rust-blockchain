use uuid::Uuid;

#[derive(Debug)]
pub struct Wallet{
    address: String,
    balance: f64,
}

impl Wallet{
    pub fn new() ->Self{
        Wallet{
            address: Uuid::new_v4().to_string(),
            balance: 10.0,
        }
    }
}