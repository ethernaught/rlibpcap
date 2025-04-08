use std::{fmt, io};

#[derive(Clone, Copy, Eq, PartialEq)]
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

    pub fn is_broadcast(&self) -> bool {
        self.address == [255; 6]
    }

    pub fn to_string(&self) -> String {
        format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.address[0],
            self.address[1],
            self.address[2],
            self.address[3],
            self.address[4],
            self.address[5]
        )
    }
}

impl TryFrom<&[u8]> for EthernetAddress {

    type Error = io::Error;

    fn try_from(value: &[u8]) -> io::Result<Self> {
        if value.len() != 6 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "MAC address must be 6 bytes"));
        }

        let mut address = [0u8; 6];
        address.copy_from_slice(&value[0..6]);
        Ok(Self {
            address
        })
    }
}

impl fmt::Debug for EthernetAddress {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EthernetAddress {{ address: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X} }}",
            self.address[0],
            self.address[1],
            self.address[2],
            self.address[3],
            self.address[4],
            self.address[5]
        )
    }
}
