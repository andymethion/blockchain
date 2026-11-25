use std::fmt;

use anyhow::Result;
use ed25519_dalek::{Verifier, VerifyingKey};

use crate::primitives::Address;

pub struct Signature(pub [u8; 64]);

impl Signature {
    pub fn verify(&self, address: &Address, msg: &[u8]) -> Result<()> {
        let verifying_key = VerifyingKey::from_bytes(&address.0)?;
        let signature = ed25519_dalek::Signature::from_bytes(&self.0);
        verifying_key.verify(msg, &signature)?;
        Ok(())
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}
