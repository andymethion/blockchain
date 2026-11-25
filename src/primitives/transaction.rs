use crate::primitives::{Address, Amount, Hash, Signature};

pub struct Transaction {
    pub hash: Hash,
    pub sender: Address,
    pub recipient: Address,
    pub amount: Amount,
    pub fee: Amount,
    pub nonce: u64,
    pub signature: Signature,
}
