#[derive(Debug, PartialEq , serde::Serialize, serde::Deserialize)]
pub enum DigestId {
    Sha256([u8; 32]),
}