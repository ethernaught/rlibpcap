
use std::any::Any;
use crate::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use crate::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;

const SLL2_FRAME_LEN: usize = 20;

#[derive(Clone, Debug)]
pub struct Sll2Frame {
    protocol: u16,
    reserved: u16,
    if_index: u32,
    hatype: u16,
    pkttype: u16,
    halen: u16,
    addr: [u8; 6],
    data: Option<Box<dyn Layer>>,
    length: usize
}

impl Sll2Frame {

    pub fn new() -> Self {
        Self {
            protocol: 0,
            reserved: 0,
            if_index: 0,
            hatype: 0,
            pkttype: 0,
            halen: 0,
            addr: [0u8; 6],
            data: None,
            length: SLL2_FRAME_LEN
        }
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

        Some(Self {
            protocol: u16::from_be_bytes(buf[0..2].try_into().unwrap()),
            reserved: u16::from_be_bytes(buf[2..4].try_into().unwrap()),
            if_index: u32::from_be_bytes(buf[4..8].try_into().unwrap()),
            hatype: u16::from_be_bytes(buf[8..10].try_into().unwrap()),
            pkttype: u16::from_be_bytes(buf[10..12].try_into().unwrap()),
            halen: u16::from_be_bytes(buf[12..14].try_into().unwrap()),
            addr: buf[14..20].try_into().unwrap(),
            data: None,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; SLL2_FRAME_LEN];
        buf.splice(0..2, self.protocol.to_be_bytes());
        buf.splice(2..4, self.reserved.to_be_bytes());
        buf.splice(4..8, self.if_index.to_be_bytes());
        buf.splice(8..10, self.hatype.to_be_bytes());
        buf.splice(10..12, self.pkttype.to_be_bytes());
        buf.splice(12..14, self.halen.to_be_bytes());
        //buf.splice(0..6, self.addr.to_bytes());

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
