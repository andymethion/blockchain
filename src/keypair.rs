use ed25519_dalek::{Signer, SigningKey};

use crate::primitives::{Address, Signature};

pub struct Keypair(pub SigningKey);

impl Keypair {
    pub fn generate() -> Self {
        Self(SigningKey::generate(&mut rand::rng()))
    }

    pub fn address(&self) -> Address {
        Address(self.0.verifying_key().to_bytes())
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        Signature(self.0.sign(msg).to_bytes())
    }
}
