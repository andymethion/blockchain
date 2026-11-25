use crate::primitives::{Address, Hash, Signature, Transaction};

pub struct Block {
    pub hash: Hash,
    pub parent: Hash,
    pub number: u64,
    pub timestamp: u64,
    pub validator: Address,
    pub signature: Signature,
    pub transactions: Vec<Transaction>,
}
