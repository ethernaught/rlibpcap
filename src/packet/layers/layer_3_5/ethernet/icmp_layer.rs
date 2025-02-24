use std::any::Any;
use std::net::Ipv4Addr;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::layer_3::ethernet::inter::protocols::Protocols;

#[derive(Clone, Debug)]
pub struct IcmpLayer {
    pub icmp_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence: u16
}

impl IcmpLayer {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 8 {
            return None;
        }

        Some(Self {
            icmp_type: buf[0],
            code: buf[1],
            checksum: u16::from_be_bytes([buf[2], buf[3]]),
            identifier: u16::from_be_bytes([buf[4], buf[5]]),
            sequence: u16::from_be_bytes([buf[6], buf[7]])
        })
    }
}

impl Layer for IcmpLayer {

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; self.len()];

        buf[0] = self.icmp_type;
        buf[1] = self.code;
        buf.splice(2..4, self.checksum.to_be_bytes());
        buf.splice(4..6, self.identifier.to_be_bytes());
        buf.splice(6..8, self.sequence.to_be_bytes());

        buf
    }

    fn len(&self) -> usize {
        8
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Layer> {
        Box::new(self.clone())
    }
}
