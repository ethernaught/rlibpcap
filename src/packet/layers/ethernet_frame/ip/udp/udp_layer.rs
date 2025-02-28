use std::any::Any;
use std::net::IpAddr;
use crate::packet::layers::ethernet_frame::ip::inter::protocols::Protocols;
use crate::packet::layers::ethernet_frame::ip::inter::utils::calculate_checksum;
use crate::packet::layers::ethernet_frame::ip::udp::inter::udp_payloads::UdpPayloads;
use crate::packet::layers::ethernet_frame::ip::udp::inter::udp_types::UdpTypes;
use crate::packet::layers::inter::layer::Layer;

const UDP_HEADER_SIZE: usize = 8;

#[derive(Clone, Debug)]
pub struct UdpLayer {
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16,
    payload: UdpPayloads
}

impl UdpLayer {

    pub fn get_source_port(&self) -> u16 {
        self.source_port
    }

    pub fn get_destination_port(&self) -> u16 {
        self.destination_port
    }

    pub fn get_length(&self) -> u16 {
        self.length
    }

    fn calculate_checksum(&self, source_address: IpAddr, destination_address: IpAddr) -> u16 {
        let mut buf = vec![0; UDP_HEADER_SIZE];
        buf.splice(0..2, self.source_port.to_be_bytes());
        buf.splice(2..4, self.destination_port.to_be_bytes());
        buf.splice(4..6, self.length.to_be_bytes());

        match source_address {
            IpAddr::V4(ip) => {
                buf.extend_from_slice(&ip.octets());
            }
            IpAddr::V6(ip) => {
                buf.extend_from_slice(&ip.octets());
            }
        }

        match destination_address {
            IpAddr::V4(ip) => {
                buf.extend_from_slice(&ip.octets());
            }
            IpAddr::V6(ip) => {
                buf.extend_from_slice(&ip.octets());
            }
        }

        buf.push(0);
        buf.push(Protocols::Udp.get_code());
        buf.extend_from_slice(&self.length.to_be_bytes());

        match &self.payload {
            UdpPayloads::Known(_, payload) => {
                buf.extend(payload.to_bytes());
            }
            UdpPayloads::Unknown(payload) => {
                buf.extend(payload);
            }
        }

        calculate_checksum(&buf)
    }

    pub fn compute_checksum(&mut self, source_address: IpAddr, destination_address: IpAddr) -> u16 {
        let checksum = self.calculate_checksum(source_address, destination_address);
        self.checksum = checksum;
        checksum
    }

    pub fn validate_checksum(&self, source_address: IpAddr, destination_address: IpAddr) -> bool {
        self.checksum == self.calculate_checksum(source_address, destination_address)
    }

    pub fn get_checksum(&self) -> u16 {
        self.checksum
    }

    pub fn get_type(&self) -> UdpTypes {
        match self.payload {
            UdpPayloads::Known(_type, _) => {
                _type
            }
            UdpPayloads::Unknown(_) => {
                UdpTypes::Unknown
            }
        }
    }

    pub fn set_payload(&mut self, _type: UdpTypes, layer: Box<dyn Layer>) {
        //self.payload = payload;
    }

    pub fn get_payload(&self) -> &UdpPayloads {
        &self.payload
    }

    pub fn get_payload_mut(&mut self) -> &mut UdpPayloads {
        &mut self.payload
    }
}

impl Layer for UdpLayer {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < UDP_HEADER_SIZE {
            return None;
        }

        Some(Self {
            source_port: u16::from_be_bytes([buf[0], buf[1]]),
            destination_port: u16::from_be_bytes([buf[2], buf[3]]),
            length: u16::from_be_bytes([buf[4], buf[5]]),
            checksum: u16::from_be_bytes([buf[6], buf[7]]),
            payload: UdpPayloads::get_type_from_buf(&buf[8..])
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; UDP_HEADER_SIZE];

        buf.splice(0..2, self.source_port.to_be_bytes());
        buf.splice(2..4, self.destination_port.to_be_bytes());
        buf.splice(4..6, self.length.to_be_bytes());
        buf.splice(6..8, self.checksum.to_be_bytes());

        match &self.payload {
            UdpPayloads::Known(_, payload) => {
                buf.extend(payload.to_bytes());
            }
            UdpPayloads::Unknown(payload) => {
                buf.extend(payload);
            }
        }

        buf
    }

    fn len(&self) -> usize {
        self.length as usize
    }

    fn compute_length(&mut self) -> usize {
        let length = match &mut self.payload {
            UdpPayloads::Known(_, payload) => {
                payload.len() + UDP_HEADER_SIZE
            }
            UdpPayloads::Unknown(payload) => {
                payload.len() + UDP_HEADER_SIZE
            }
        };

        self.length = length as u16;
        length
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
