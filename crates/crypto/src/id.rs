use serde::{Deserialize, Serialize};
use sha2::{digest::Digest, Sha256};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum DigestId {
    Sha256([u8; 32]),
}

impl DigestId {
    pub fn ensure(&self, content: &[u8]) -> Result<(), &'static str> {
        match self {
            DigestId::Sha256(digest) => {
                let expected: [u8; 32] = Sha256::digest(content).try_into().map_err(|_| "HASH_ERROR")?;
                if &expected == digest {
                    return Err("DIGEST_ERROR")
                }
            }
        }
        Ok(())
    }
}
