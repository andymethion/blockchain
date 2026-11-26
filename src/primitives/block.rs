use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    keypair::Keypair,
    primitives::{Address, Hash, Signature, Transaction},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    pub parent: Hash,
    pub hash: Hash,
    pub number: u64,
    pub timestamp: u64,
    pub validator: Address,
    pub signature: Signature,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        keypair: &Keypair,
        parent: Hash,
        number: u64,
        timestamp: u64,
        transactions: Vec<Transaction>,
    ) -> Result<Self> {
        let mut block = Self {
            parent,
            hash: Hash::default(),
            number,
            timestamp,
            validator: keypair.address(),
            signature: Signature::default(),
            transactions,
        };

        block.hash = block.hash()?;
        block.signature = keypair.sign(&block.hash.0);

        Ok(block)
    }

    pub fn genesis(keypair: &Keypair, timestamp: u64) -> Result<Self> {
        Self::new(keypair, Hash::default(), 0, timestamp, Vec::default())
    }

    pub fn hash(&self) -> Result<Hash> {
        Ok(Hash::hash(&postcard::to_stdvec(&(
            &self.parent,
            &self.number,
            &self.timestamp,
            &self.validator,
            &self.transactions,
        ))?))
    }

    pub fn verify(&self) -> Result<()> {
        if self.hash != self.hash()? {
            anyhow::bail!("Invalid block hash");
        }

        self.signature.verify(&self.validator, &self.hash.0)?;
        self.transactions.iter().try_for_each(Transaction::verify)?;

        Ok(())
    }
}
