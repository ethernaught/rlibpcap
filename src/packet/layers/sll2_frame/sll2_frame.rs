use std::any::Any;
use crate::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::sll2_frame::inter::packet_types::PacketTypes;

const SLL2_FRAME_LEN: usize = 14;

#[derive(Clone, Debug)]
pub struct Sll2Frame {
    family: u16,
    protocol: u16,//EthernetTypes,
    if_index: i32,
    data_link_type: u16,
    packet_type: u8,//PacketTypes,
    address: [u8; 8],
    data: Option<Box<dyn Layer>>,
    length: usize
}

impl Sll2Frame {

    /*
    pub fn new(protocol: EthernetTypes, packet_type: PacketTypes) -> Self {
        Self {
            protocol,
            reserved: 0,
            if_index: 0,
            hatype: 0,
            packet_type,
            addrress: Vec::new(),
            data: None,
            length: SLL2_FRAME_LEN
        }
    }*/

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

        //address_length: u16::from_be_bytes(buf[12..14].try_into().unwrap()),

        //SockAddrLl { sll_family: 17, sll_protocol: 8, sll_ifindex: 3, sll_hatype: 1, sll_pkttype: 0, sll_halen: 6, sll_addr: [60, 82, 161, 18, 164, 80, 0, 0] }

        /*
        let ssl = Self {
            family: u16::from_be_bytes(buf[0..2].try_into().unwrap()),
            protocol: u16::from_be_bytes(buf[2..4].try_into().unwrap()),
            if_index: i32::from_be_bytes(buf[4..8].try_into().unwrap()),
            data_link_type: u16::from_be_bytes(buf[8..10].try_into().unwrap()),
            pkttype: buf[10],
            halen: buf[11],
            addr: [0u8; 8],
        };*/

        Some(Self {
            family: u16::from_be_bytes(buf[0..2].try_into().unwrap()),
            protocol: u16::from_be_bytes(buf[2..4].try_into().unwrap()),
            if_index: i32::from_be_bytes(buf[4..8].try_into().unwrap()),
            data_link_type: u16::from_be_bytes(buf[8..10].try_into().unwrap()),
            packet_type: buf[11],//PacketTypes::from_code(u16::from_be_bytes(buf[10..12].try_into().unwrap())).unwrap(),
            address: [0u8; 8],//buf[14..20].try_into().unwrap(),
            data: None,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; SLL2_FRAME_LEN];
        /*
        buf.splice(0..2, self.protocol.get_code().to_be_bytes());
        buf.splice(2..4, self.reserved.to_be_bytes());
        buf.splice(4..8, self.if_index.to_be_bytes());
        buf.splice(8..10, self.hatype.to_be_bytes());
        buf.splice(10..12, self.packet_type.get_code().to_be_bytes());*/
        //buf.splice(12..14, self.halen.to_be_bytes());
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
