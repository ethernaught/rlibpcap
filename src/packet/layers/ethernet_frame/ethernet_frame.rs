use std::any::Any;
use crate::packet::layers::ethernet_frame::arp::arp_extension::ArpLayer;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::packet::layers::ethernet_frame::inter::types::Types;
use crate::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;

const ETHERNET_FRAME_LEN: usize = 14;

#[derive(Clone, Debug)]
pub struct EthernetFrame {
    destination: EthernetAddress,
    source: EthernetAddress,
    _type: Types,
    data: Option<Box<dyn Layer>>,
    length: usize
}

impl EthernetFrame {

    pub fn new(destination: EthernetAddress, source: EthernetAddress, _type: Types) -> Self {
        Self {
            destination,
            source,
            _type,
            data: None,
            length: ETHERNET_FRAME_LEN
        }
    }

    pub fn set_destination(&mut self, destination: EthernetAddress) {
        self.destination = destination;
    }

    pub fn get_destination(&self) -> EthernetAddress {
        self.destination
    }

    pub fn set_source(&mut self, source: EthernetAddress) {
        self.source = source;
    }

    pub fn get_source(&self) -> EthernetAddress {
        self.source
    }

    pub fn set_type(&mut self, _type: Types) {
        self._type = _type;
    }

    pub fn get_type(&self) -> Types {
        self._type
    }

    pub fn set_data(&mut self, data: Box<dyn Layer>) {
        self.data = Some(data);
    }

    pub fn get_data(&self) -> Option<&Box<dyn Layer>> {
        self.data.as_ref()
    }
}

impl Layer for EthernetFrame {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < ETHERNET_FRAME_LEN {
            return None;
        }

        let _type = Types::get_type_from_code(u16::from_be_bytes([buf[12], buf[13]])).unwrap();

        let data = match _type {
            Types::IPv4 => {
                Some(Ipv4Layer::from_bytes(&buf[ETHERNET_FRAME_LEN..])?.dyn_clone())
            }
            Types::Arp => {
                Some(ArpLayer::from_bytes(&buf[ETHERNET_FRAME_LEN..])?.dyn_clone())
            }
            Types::IPv6 => {
                Some(Ipv6Layer::from_bytes(&buf[ETHERNET_FRAME_LEN..])?.dyn_clone())
            }
            Types::Broadcast => {
                None
            }
        };

        Some(Self {
            destination: EthernetAddress::new(buf[0], buf[1], buf[2], buf[3], buf[4], buf[5]),
            source: EthernetAddress::new(buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]),
            _type,
            data,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; ETHERNET_FRAME_LEN];
        buf.splice(0..6, self.destination.to_bytes());
        buf.splice(6..12, self.source.to_bytes());
        buf.splice(12..14, self._type.get_code().to_be_bytes());

        match &self.data {
            Some(data) => {
                buf.extend(data.to_bytes());
            }
            None => {}
        }

        buf
    }

    fn len(&self) -> usize {
        self.length
    }

    fn compute_length(&mut self) -> usize {
        let length = match &mut self.data {
            Some(layer) => {
                layer.compute_length() + ETHERNET_FRAME_LEN
            }
            None => {
                ETHERNET_FRAME_LEN
            }
        };

        self.length = length;
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
