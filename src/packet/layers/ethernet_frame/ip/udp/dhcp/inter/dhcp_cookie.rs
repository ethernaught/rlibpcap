#[derive(Clone, Copy, Debug)]
pub struct DhcpCookie {
    magic: [u8; 4]
}

impl DhcpCookie {

    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            magic: [a, b, c, d]
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        self.magic
    }
}
