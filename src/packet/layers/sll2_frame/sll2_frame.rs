use std::any::Any;
use crate::utils::data_link_types::DataLinkTypes;
use crate::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use crate::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use crate::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::sll2_frame::inter::packet_types::PacketTypes;

pub const SLL2_FRAME_LEN: usize = 20;

#[derive(Clone, Debug)]
pub struct Sll2Frame {
    protocol: EthernetTypes,
    reserved: u16,
    if_index: i32,
    data_link_type: DataLinkTypes,
    packet_type: PacketTypes,
    address_length: u8,
    address: [u8; 8],
    data: Option<Box<dyn Layer>>,
    length: usize
}

impl Sll2Frame {

    pub fn new(address: [u8; 8], address_length: u8, protocol: EthernetTypes, packet_type: PacketTypes) -> Self {
        Self {
            protocol,
            reserved: 0,
            if_index: 1,
            data_link_type: DataLinkTypes::Null,
            packet_type,
            address_length,
            address,
            data: None,
            length: SLL2_FRAME_LEN
        }
    }

    pub fn get_protocol(&self) -> EthernetTypes {
        self.protocol
    }

    pub fn set_protocol(&mut self, protocol: EthernetTypes) {
        self.protocol = protocol;
    }

    pub fn get_if_index(&self) -> i32 {
        self.if_index
    }

    pub fn set_if_index(&mut self, if_index: i32) {
        self.if_index = if_index;
    }

    pub fn get_data_link_type(&self) -> DataLinkTypes {
        self.data_link_type
    }

    pub fn set_data_link_type(&mut self, data_link_type: DataLinkTypes) {
        self.data_link_type = data_link_type;
    }

    pub fn get_packet_type(&self) -> PacketTypes {
        self.packet_type
    }

    pub fn set_packet_type(&mut self, packet_type: PacketTypes) {
        self.packet_type = packet_type;
    }

    pub fn get_address_length(&self) -> u8 {
        self.address_length
    }

    pub fn set_address_length(&mut self, address_length: u8) {
        self.address_length = address_length;
    }

    pub fn get_address(&self) -> &[u8; 8] {
        &self.address
    }

    pub fn set_address(&mut self, address: [u8; 8]) {
        self.address = address;
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

impl Layer for Sll2Frame {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < SLL2_FRAME_LEN {
            return None;
        }

        let protocol = EthernetTypes::from_code(u16::from_be_bytes(buf[0..2].try_into().unwrap())).unwrap();

        let data = match protocol {
            EthernetTypes::Ipv4 => {
                Some(Ipv4Layer::from_bytes(&buf[SLL2_FRAME_LEN..])?.dyn_clone())
            }
            EthernetTypes::Arp => {
                Some(ArpExtension::from_bytes(&buf[SLL2_FRAME_LEN..])?.dyn_clone())
            }
            EthernetTypes::Ipv6 => {
                Some(Ipv6Layer::from_bytes(&buf[SLL2_FRAME_LEN..])?.dyn_clone())
            }
            EthernetTypes::Broadcast => {
                None
            }
        };

        Some(Self {
            protocol,
            reserved: u16::from_be_bytes(buf[2..4].try_into().unwrap()),
            if_index: i32::from_be_bytes(buf[4..8].try_into().unwrap()),
            data_link_type: DataLinkTypes::from_code(u16::from_be_bytes(buf[8..10].try_into().unwrap()) as u32).unwrap(),
            packet_type: PacketTypes::from_code(buf[10]).unwrap(),
            address_length: buf[11],
            address: buf[12..20].try_into().unwrap(),
            data,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; SLL2_FRAME_LEN];
        buf.splice(0..2, self.protocol.get_code().to_be_bytes());
        buf.splice(4..8, self.if_index.to_be_bytes());
        buf.splice(8..10, self.data_link_type.get_code().to_be_bytes());
        buf[10] = self.packet_type.get_code();
        buf[11] = self.address_length;

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
                layer.len() + SLL2_FRAME_LEN
            }
            None => {
                SLL2_FRAME_LEN
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
