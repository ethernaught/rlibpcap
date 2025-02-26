#[derive(Clone, Copy, Debug)]
pub struct DHCPCookie {
    magic: [u8; 4]
}

impl DHCPCookie {

    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self {
            magic: [a, b, c, d]
        }
    }
}
