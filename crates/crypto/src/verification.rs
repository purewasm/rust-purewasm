use serde::{Serialize, Deserialize};
use purewasm_misc::serde_bytes::serde_bytes_array;

const WOTS_PUBKEY_SIZE: usize = 32;
const WOTS_SIG_SIZE: usize = 1340;
const ED25519_PUBKEY_SIZE: usize = 32;
const ED25519_SIG_SIZE: usize = 64;

#[derive(Serialize, Deserialize, Debug)]
pub enum IdPublicKeyKind {
    Winternitz([u8; WOTS_PUBKEY_SIZE]),
    Ed25519([u8; ED25519_PUBKEY_SIZE]),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IdSignatureKind {
    #[serde(with = "serde_bytes_array")]
    Winternitz([u8; WOTS_SIG_SIZE]),
    #[serde(with = "serde_bytes_array")]
    Ed25519([u8; ED25519_SIG_SIZE]),
}


impl IdPublicKeyKind {
    pub fn verify(&self, sig: IdSignatureKind) -> Result<bool, &'static str> {
        match &self {
            IdPublicKeyKind::Winternitz(pk) => {
                if let IdSignatureKind::Winternitz(sig) = sig {

                }
                return Err("Signature and public");
            },
            IdPublicKeyKind::Ed25519(_) => unimplemented!(),
        }
    }
}
