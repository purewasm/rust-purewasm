pub struct Channel {
    pub id: DigestId,
    pub organizations: Vec<Organization>,
    pub rotation_msg_size: u8
}