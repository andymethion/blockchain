use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    keypair::Keypair,
    primitives::{Address, Amount, Hash, Signature},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: Hash,
    pub sender: Address,
    pub recipient: Address,
    pub amount: Amount,
    pub fee: Amount,
    pub nonce: u64,
    pub signature: Signature,
}

impl Transaction {
    pub fn new(
        keypair: &Keypair,
        recipient: Address,
        amount: Amount,
        fee: Amount,
        nonce: u64,
    ) -> Result<Self> {
        let mut transaction = Transaction {
            hash: Hash::default(),
            sender: keypair.address(),
            recipient,
            amount,
            fee,
            nonce,
            signature: Signature::default(),
        };

        transaction.hash = transaction.hash()?;
        transaction.signature = keypair.sign(&transaction.hash.0);

        Ok(transaction)
    }

    pub fn hash(&self) -> Result<Hash> {
        Ok(Hash::hash(&postcard::to_stdvec(&(
            &self.sender,
            &self.recipient,
            &self.amount,
            &self.fee,
            &self.nonce,
        ))?))
    }

    pub fn verify(&self) -> Result<()> {
        match self.hash == self.hash()? {
            true => self.signature.verify(&self.sender, &self.hash.0),
            false => anyhow::bail!("Invalid transaction hash"),
        }
    }
}
