use crate::proof::IdProof;
use purewasm_codec::cbor::CborCodec;
use purewasm_core::{Codec, DigestId};
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use serde::{Deserialize, Serialize};
use alloc::{collections::BTreeMap, vec::Vec};
use wasmid_core::PersistedIdEvent;

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    pub id: DigestId,
    pub events: BTreeMap<DigestId, IdProof>,
}

pub struct InceptionEvents {
    pub elements: Vec<(DigestId, PersistedIdEvent)>,
    pub tree: MerkleTree<Sha256>,
}

impl InceptionEvents {
    pub fn add_event(&mut self, id: DigestId, event: PersistedIdEvent) {
        let e = (id, event);
        self.tree
            .insert(Sha256::hash(&CborCodec.to_bytes(&e).unwrap()));
        self.elements.push(e);
    }

    pub fn gen_proof(&mut self) -> ([u8; 32], Vec<Vec<u8>>) {
        let mut proofs: Vec<Vec<u8>> = Vec::new();
        self.tree.commit();
        let merkle_root = self
            .tree
            .root()
            .ok_or("couldn't get the merkle root")
            .unwrap();
        for (i, _) in self.elements.iter().enumerate() {
            let proof = self.tree.proof(&[i]).to_bytes();
            proofs.push(proof);
        }
        (merkle_root, proofs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rs_merkle::{MerkleProof, MerkleTree};
    #[test]
    fn add_event_test() {
        let mut c = InceptionEvents {
            elements: vec![],
            tree: MerkleTree::<Sha256>::new(),
        };
        for i  in 1u8..8{
            c.add_event(
                DigestId::Sha256([i; 32]),
                PersistedIdEvent {
                    context: DigestId::Sha256([i; 32]),
                    command: vec![i],
                    event: vec![i],
                },
            );
        }
        
        let r = c.gen_proof();
        eprintln!("Root: {:?}", r.0);
        for (i, b) in r.1.iter().enumerate() {
            eprintln!("Proof: {:?}", b);
            let proof = MerkleProof::<Sha256>::try_from(b.to_owned()).unwrap();
            let is_valid = proof.verify(
                r.0,
                &[i],
                &[Sha256::hash(&CborCodec.to_bytes(&c.elements[i]).unwrap())],
                r.1.len(),
            );
            eprintln!("Valid: {}", is_valid);
        }
    }
}
