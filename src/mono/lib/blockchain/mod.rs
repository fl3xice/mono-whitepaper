pub struct Transaction {
    pub address: String,
    pub from: String,
    pub amount: f64,
    pub hash_transaction: String,
    pub timestamp: u64,
}

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}
