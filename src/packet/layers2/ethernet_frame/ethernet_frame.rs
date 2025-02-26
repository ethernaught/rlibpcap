use std::any::Any;
use crate::packet::layers2::ethernet_frame::arp::arp_extension::ArpLayer;
use crate::packet::layers2::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::packet::layers2::ethernet_frame::inter::types::Types;
use crate::packet::layers2::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers2::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers2::inter::layer::Layer;

#[derive(Clone, Debug)]
pub struct EthernetFrame {
    destination: EthernetAddress,
    source: EthernetAddress,
    _type: Types,
    data: Option<Box<dyn Layer>>
}

impl EthernetFrame {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 14 {
            return None;
        }

        let _type = Types::get_type_from_code(u16::from_be_bytes([buf[12], buf[13]])).unwrap();

        let data = match _type {
            Types::IPv4 => {
                Some(Ipv4Layer::from_bytes(&buf[14..])?.dyn_clone())
            }
            Types::Arp => {
                Some(ArpLayer::from_bytes(&buf[14..])?.dyn_clone())
            }
            Types::IPv6 => {
                Some(Ipv6Layer::from_bytes(&buf[14..])?.dyn_clone())
            }
            Types::Broadcast => {
                None
            }
        };

        Some(Self {
            destination: EthernetAddress::new(buf[0], buf[1], buf[2], buf[3], buf[4], buf[5]),
            source: EthernetAddress::new(buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]),
            _type,
            data
        })
    }

    pub fn get_destination(&self) -> EthernetAddress {
        self.destination
    }

    pub fn get_source(&self) -> EthernetAddress {
        self.source
    }

    pub fn get_type(&self) -> Types {
        self._type
    }
}

impl Layer for EthernetFrame {

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; self.len()];
        buf.splice(0..6, self.destination.to_bytes());
        buf.splice(6..12, self.source.to_bytes());
        buf.splice(12..14, self._type.get_code().to_be_bytes());

        buf
    }

    fn len(&self) -> usize {
        14
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
