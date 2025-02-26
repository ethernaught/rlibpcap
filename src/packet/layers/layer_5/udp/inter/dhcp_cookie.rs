#[derive(Clone, Copy, Debug)]
pub struct DHCPCookie {
    magic: [u8; 4]
}

impl DHCPCookie {

    pub fn from_bytes(magic: [u8; 4]) -> Self {
        Self {
            magic
        }
    }
}
