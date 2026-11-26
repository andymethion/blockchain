use std::fmt;

use anyhow::Result;
use ed25519_dalek::{Verifier, VerifyingKey};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

use crate::primitives::Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Default for Signature {
    fn default() -> Self {
        Self([0; 64])
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(
            Vec::<u8>::deserialize(deserializer)?
                .try_into()
                .map_err(|_| D::Error::custom(""))?,
        ))
    }
}
