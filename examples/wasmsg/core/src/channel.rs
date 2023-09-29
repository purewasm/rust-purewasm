pub struct Channel {
    pub id: String,
    pub notaries: Vec<Organization>,
    pub signers: Vec<Organization>,
    pub rotation_msg_size: u8
}