#[derive(Clone, Copy, Debug)]
pub struct EthernetAddress {
    address: [u8; 6]
}

impl EthernetAddress {

    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> EthernetAddress {
        Self {
            address: [a, b, c, d, e, f]
        }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        self.address
    }

    pub fn to_string(&self) -> String {
        self.address.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join(":")
    }
}

