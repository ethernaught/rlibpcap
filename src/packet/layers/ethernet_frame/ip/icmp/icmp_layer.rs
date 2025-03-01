use std::any::Any;
use crate::packet::layers::inter::layer::Layer;

const ICMP_PAYLOAD_SIZE: usize = 8;

#[derive(Clone, Debug)]
pub struct IcmpLayer {
    _type: u8,
    code: u8,
    checksum: u16,
    identifier: u16,
    sequence_number: u16,
    data: Vec<u8>,
    length: usize
}

impl IcmpLayer {

    pub fn get_type(&self) -> u8 {
        self._type
    }

    pub fn get_code(&self) -> u8 {
        self.code
    }

    pub fn get_checksum(&self) -> u16 {
        self.checksum
    }

    pub fn get_identifier(&self) -> u16 {
        self.identifier
    }

    pub fn get_sequence_number(&self) -> u16 {
        self.sequence_number
    }
}

impl Layer for IcmpLayer {
    
    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < ICMP_PAYLOAD_SIZE {
            return None;
        }

        Some(Self {
            _type: buf[0],
            code: buf[1],
            checksum: u16::from_be_bytes([buf[2], buf[3]]),
            identifier: u16::from_be_bytes([buf[4], buf[5]]),
            sequence_number: u16::from_be_bytes([buf[6], buf[7]]),
            data: buf[8..].to_vec(),
            length: buf.len(),
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; ICMP_PAYLOAD_SIZE];

        buf[0] = self._type;
        buf[1] = self.code;
        buf.splice(2..4, self.checksum.to_be_bytes());
        buf.splice(4..6, self.identifier.to_be_bytes());
        buf.splice(6..8, self.sequence_number.to_be_bytes());
        buf.extend_from_slice(&self.data);

        buf
    }

    fn len(&self) -> usize {
        self.length
    }

    fn compute_length(&mut self) -> usize {
        let length = ICMP_PAYLOAD_SIZE + self.data.len();
        self.length = length;
        self.length
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
