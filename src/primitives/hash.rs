use std::fmt;

pub struct Hash(pub [u8; 32]);

impl Hash {
    pub fn hash(input: &[u8]) -> Self {
        Self(*blake3::hash(input).as_bytes())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}
