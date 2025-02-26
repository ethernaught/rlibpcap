use std::any::Any;
use std::fmt::Debug;
use std::net::Ipv4Addr;
use crate::packet::layers2::inter::layer::Layer;
use crate::packet::layers::layer_2::inter::ethernet_address::EthernetAddress;

#[derive(Clone, Debug)]
pub struct ArpLayer {
    hardware_type: u16,
    protocol_type: u16,
    hardware_size: u8,
    protocol_size: u8,
    opcode: u16,
    sender_mac: EthernetAddress,
    sender_ip: Ipv4Addr,
    target_mac: EthernetAddress,
    target_ip: Ipv4Addr,
}

impl ArpLayer {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 28 {
            return None;
        }

        Some(Self {
            hardware_type: u16::from_be_bytes([buf[0], buf[1]]),
            protocol_type: u16::from_be_bytes([buf[2], buf[3]]),
            hardware_size: buf[4],
            protocol_size: buf[5],
            opcode: u16::from_be_bytes([buf[6], buf[7]]),
            sender_mac: EthernetAddress::new(buf[8], buf[9], buf[10], buf[11], buf[12], buf[13]),
            sender_ip: Ipv4Addr::new(buf[14], buf[15], buf[16], buf[17]),
            target_mac: EthernetAddress::new(buf[18], buf[19], buf[20], buf[21], buf[22], buf[23]),
            target_ip: Ipv4Addr::new(buf[24], buf[25], buf[26], buf[27]),
        })
    }

    pub fn get_hardware_type(&self) -> u16 {
        self.hardware_type
    }

    pub fn get_protocol_type(&self) -> u16 {
        self.protocol_type
    }

    pub fn get_hardware_size(&self) -> u8 {
        self.hardware_size
    }

    pub fn get_protocol_size(&self) -> u8 {
        self.protocol_size
    }

    pub fn get_opcode(&self) -> u16 {
        self.opcode
    }

    pub fn get_sender_mac(&self) -> EthernetAddress {
        self.sender_mac
    }

    pub fn get_sender_ip(&self) -> Ipv4Addr {
        self.sender_ip
    }

    pub fn get_target_mac(&self) -> EthernetAddress {
        self.target_mac
    }

    pub fn get_target_ip(&self) -> Ipv4Addr {
        self.target_ip
    }
}

impl Layer for ArpLayer {

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; self.len()];

        buf.splice(0..2, self.hardware_type.to_be_bytes());
        buf.splice(2..4, self.protocol_type.to_be_bytes());
        buf[4] = self.hardware_size;
        buf[5] = self.protocol_size;
        buf.splice(6..8, self.opcode.to_be_bytes());
        buf.splice(8..14, self.sender_mac.to_bytes());
        buf.splice(14..18, self.sender_ip.octets());
        buf.splice(18..24, self.target_mac.to_bytes());
        buf.splice(24..28, self.target_ip.octets());

        buf
    }

    fn len(&self) -> usize {
        28
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
