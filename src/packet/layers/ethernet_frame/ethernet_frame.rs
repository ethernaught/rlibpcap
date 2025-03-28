use std::any::Any;
use crate::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use crate::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;

const ETHERNET_FRAME_LEN: usize = 14;

#[derive(Clone, Debug)]
pub struct EthernetFrame {
    destination_mac: EthernetAddress,
    source_mac: EthernetAddress,
    _type: EthernetTypes,
    data: Option<Box<dyn Layer>>,
    length: usize
}

impl EthernetFrame {

    pub fn new(destination_mac: EthernetAddress, source_mac: EthernetAddress, _type: EthernetTypes) -> Self {
        Self {
            destination_mac,
            source_mac,
            _type,
            data: None,
            length: ETHERNET_FRAME_LEN
        }
    }

    pub fn set_destination_mac(&mut self, destination_mac: EthernetAddress) {
        self.destination_mac = destination_mac;
    }

    pub fn get_destination_mac(&self) -> EthernetAddress {
        self.destination_mac
    }

    pub fn set_source_mac(&mut self, source_mac: EthernetAddress) {
        self.source_mac = source_mac;
    }

    pub fn get_source_mac(&self) -> EthernetAddress {
        self.source_mac
    }

    pub fn set_type(&mut self, _type: EthernetTypes) {
        self._type = _type;
    }

    pub fn get_type(&self) -> EthernetTypes {
        self._type
    }

    pub fn set_data(&mut self, data: Box<dyn Layer>) {
        self.length = data.len();
        self.data = Some(data);
    }

    pub fn get_data(&self) -> Option<&Box<dyn Layer>> {
        self.data.as_ref()
    }

    pub fn get_data_mut(&mut self) -> Option<&mut Box<dyn Layer>> {
        self.data.as_mut()
    }
}

impl Layer for EthernetFrame {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < ETHERNET_FRAME_LEN {
            return None;
        }

        let _type = EthernetTypes::from_code(u16::from_be_bytes([buf[12], buf[13]])).unwrap();

        let data = match _type {
            EthernetTypes::Ipv4 => {
                Some(Ipv4Layer::from_bytes(&buf[ETHERNET_FRAME_LEN..])?.dyn_clone())
            }
            EthernetTypes::Arp => {
                Some(ArpExtension::from_bytes(&buf[ETHERNET_FRAME_LEN..])?.dyn_clone())
            }
            EthernetTypes::Ipv6 => {
                Some(Ipv6Layer::from_bytes(&buf[ETHERNET_FRAME_LEN..])?.dyn_clone())
            }
            EthernetTypes::Broadcast => {
                None
            }
        };

        Some(Self {
            destination_mac: EthernetAddress::new(buf[0], buf[1], buf[2], buf[3], buf[4], buf[5]),
            source_mac: EthernetAddress::new(buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]),
            _type,
            data,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; ETHERNET_FRAME_LEN];
        buf.splice(0..6, self.destination_mac.to_bytes());
        buf.splice(6..12, self.source_mac.to_bytes());
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
        self.length = match &self.data {
            Some(layer) => {
                layer.len() + ETHERNET_FRAME_LEN
            }
            None => {
                ETHERNET_FRAME_LEN
            }
        };

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
