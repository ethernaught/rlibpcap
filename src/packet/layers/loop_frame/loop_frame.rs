use std::any::Any;
use crate::utils::data_link_types::DataLinkTypes;
use crate::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use crate::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use crate::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::loop_frame::inter::loop_types::LoopTypes;
use crate::packet::layers::sll2_frame::inter::packet_types::PacketTypes;

pub const LOOP_FRAME_LENGTH: usize = 4;

#[derive(Clone, Debug)]
pub struct LoopFrame {
    _type: LoopTypes,
    data: Option<Box<dyn Layer>>,
    length: usize
}

impl LoopFrame {

    pub fn new(_type: LoopTypes) -> Self {
        Self {
            _type,
            data: None,
            length: LOOP_FRAME_LENGTH
        }
    }

    pub fn get_type(&self) -> LoopTypes {
        self._type
    }

    pub fn set_type(&mut self, _type: LoopTypes) {
        self._type = _type;
    }

    pub fn get_data<T: 'static>(&self) -> Option<&T> {
        self.data.as_ref()?.as_any().downcast_ref::<T>()
    }

    pub fn get_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.as_mut()?.as_any_mut().downcast_mut::<T>()
    }

    pub fn set_data(&mut self, data: Box<dyn Layer>) {
        self.length = data.len();
        self.data = Some(data);
    }
}

impl Layer for LoopFrame {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < LOOP_FRAME_LENGTH {
            return None;
        }

        let _type = LoopTypes::from_code(u32::from_ne_bytes([buf[0], buf[1], buf[2], buf[3]])).unwrap();

        let data = match _type {
            LoopTypes::Ipv4 => Some(Ipv4Layer::from_bytes(&buf[4..]).unwrap().upcast()),
            LoopTypes::Ipv6 | LoopTypes::Ipv6e2 | LoopTypes::Ipv6e3 => Some(Ipv6Layer::from_bytes(&buf[4..]).unwrap().upcast()),
            _ => None
        };

        Some(Self {
            _type,
            data,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; LOOP_FRAME_LENGTH];

        buf.splice(0..4, self._type.get_code().to_be_bytes());

        match &self.data {
            Some(data) => buf.extend(data.to_bytes()),
            None => {}
        }

        buf
    }

    fn len(&self) -> usize {
        self.length
    }

    fn compute_length(&mut self) -> usize {
        self.length = match &self.data {
            Some(layer) => layer.len() + LOOP_FRAME_LENGTH,
            None => LOOP_FRAME_LENGTH
        };

        self.length
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
