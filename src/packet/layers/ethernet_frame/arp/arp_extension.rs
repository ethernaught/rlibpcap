use std::any::Any;
use std::fmt::Debug;
use std::net::Ipv4Addr;
use crate::packet::layers::ethernet_frame::arp::inter::arp_operations::ArpOperations;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use crate::packet::layers::inter::layer::Layer;

pub const ARP_HEADER_LEN: usize = 28;

#[derive(Clone, Debug)]
pub struct ArpExtension {
    hardware_type: u16,
    protocol_type: EthernetTypes,
    hardware_size: u8,
    protocol_size: u8,
    opcode: ArpOperations,
    sender_mac: EthernetAddress,
    sender_address: Ipv4Addr,
    target_mac: EthernetAddress,
    target_address: Ipv4Addr
}

impl ArpExtension {

    pub fn new(opcode: ArpOperations, sender_mac: EthernetAddress, sender_address: Ipv4Addr, target_mac: EthernetAddress, target_address: Ipv4Addr) -> Self {
        Self {
            hardware_type: 1,
            protocol_type: EthernetTypes::Ipv4,
            hardware_size: 6,
            protocol_size: 4,
            opcode,
            sender_mac,
            sender_address,
            target_mac,
            target_address
        }
    }

    pub fn set_hardware_type(&mut self, hardware_type: u16) {
        self.hardware_type = hardware_type;
    }

    pub fn get_hardware_type(&self) -> u16 {
        self.hardware_type
    }

    pub fn set_protocol_type(&mut self, protocol_type: EthernetTypes) {
        self.protocol_type = protocol_type;
    }

    pub fn get_protocol_type(&self) -> EthernetTypes {
        self.protocol_type
    }

    pub fn set_hardware_size(&mut self, hardware_size: u8) {
        self.hardware_size = hardware_size;
    }

    pub fn get_hardware_size(&self) -> u8 {
        self.hardware_size
    }

    pub fn set_protocol_size(&mut self, protocol_size: u8) {
        self.protocol_size = protocol_size;
    }

    pub fn get_protocol_size(&self) -> u8 {
        self.protocol_size
    }

    pub fn set_opcode(&mut self, opcode: ArpOperations) {
        self.opcode = opcode;
    }

    pub fn get_opcode(&self) -> ArpOperations {
        self.opcode
    }

    pub fn set_sender_mac(&mut self, sender_mac: EthernetAddress) {
        self.sender_mac = sender_mac;
    }

    pub fn get_sender_mac(&self) -> EthernetAddress {
        self.sender_mac
    }

    pub fn set_sender_address(&mut self, sender_address: Ipv4Addr) {
        self.sender_address = sender_address;
    }

    pub fn get_sender_address(&self) -> Ipv4Addr {
        self.sender_address
    }

    pub fn set_target_mac(&mut self, target_mac: EthernetAddress) {
        self.target_mac = target_mac;
    }

    pub fn get_target_mac(&self) -> EthernetAddress {
        self.target_mac
    }

    pub fn set_target_address(&mut self, target_address: Ipv4Addr) {
        self.target_address = target_address;
    }

    pub fn get_target_address(&self) -> Ipv4Addr {
        self.target_address
    }
}

impl Layer for ArpExtension {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < ARP_HEADER_LEN {
            return None;
        }

        Some(Self {
            hardware_type: u16::from_be_bytes([buf[0], buf[1]]),
            protocol_type: EthernetTypes::from_code(u16::from_be_bytes([buf[2], buf[3]])).unwrap(),
            hardware_size: buf[4],
            protocol_size: buf[5],
            opcode: ArpOperations::from_code(u16::from_be_bytes([buf[6], buf[7]])).unwrap(),
            sender_mac: EthernetAddress::new(buf[8], buf[9], buf[10], buf[11], buf[12], buf[13]),
            sender_address: Ipv4Addr::new(buf[14], buf[15], buf[16], buf[17]),
            target_mac: EthernetAddress::new(buf[18], buf[19], buf[20], buf[21], buf[22], buf[23]),
            target_address: Ipv4Addr::new(buf[24], buf[25], buf[26], buf[27])
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; ARP_HEADER_LEN];

        buf.splice(0..2, self.hardware_type.to_be_bytes());
        buf.splice(2..4, self.protocol_type.get_code().to_be_bytes());
        buf[4] = self.hardware_size;
        buf[5] = self.protocol_size;
        buf.splice(6..8, self.opcode.get_code().to_be_bytes());
        buf.splice(8..14, self.sender_mac.to_bytes());
        buf.splice(14..18, self.sender_address.octets());
        buf.splice(18..24, self.target_mac.to_bytes());
        buf.splice(24..28, self.target_address.octets());

        buf
    }

    fn len(&self) -> usize {
        ARP_HEADER_LEN
    }

    fn compute_length(&mut self) -> usize {
        ARP_HEADER_LEN
    }

    fn dyn_clone(&self) -> Box<dyn Layer> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
