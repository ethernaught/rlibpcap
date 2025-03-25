use crate::utils::data_link_types::DataLinkTypes;
use crate::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use crate::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::loop_frame::loop_frame::LoopFrame;
use crate::packet::layers::raw_frame::raw_frame::RawFrame;
use crate::packet::layers::sll2_frame::sll2_frame::Sll2Frame;

#[derive(Debug, Clone)]
pub struct Packet {
    data_link_type: DataLinkTypes,
    frame: Box<dyn Layer>,
    frame_time: u128,
    length: usize
}

impl Packet {

    pub fn new(data_link_type: DataLinkTypes, frame_time: u128, data: &[u8]) -> Self {
        let frame = match data_link_type {
            DataLinkTypes::En10mb => {
                EthernetFrame::from_bytes(data).unwrap().dyn_clone()
            }
            DataLinkTypes::Loop => {
                LoopFrame::from_bytes(data).unwrap().dyn_clone()
            }
            DataLinkTypes::Raw => {
                RawFrame::from_bytes(data).unwrap().dyn_clone()
            }
            DataLinkTypes::Sll2 => {
                Sll2Frame::from_bytes(data).unwrap().dyn_clone()
            }
            _ => {
                todo!()
            }
        };

        Self {
            data_link_type,
            frame,
            frame_time,
            length: data.len()
        }
    }

    pub fn set_data_link_type(&mut self, data_link_type: DataLinkTypes) {
        self.data_link_type = data_link_type;
    }

    pub fn get_data_link_type(&self) -> DataLinkTypes {
        self.data_link_type
    }

    pub fn set_frame(&mut self, frame: Box<dyn Layer>) {
        self.length = frame.len();
        self.frame = frame;
    }

    pub fn get_frame(&self) -> &Box<dyn Layer> {
        &self.frame
    }

    pub fn get_frame_mut(&mut self) -> &mut Box<dyn Layer> {
        &mut self.frame
    }

    pub fn set_frame_time(&mut self, frame_time: u128) {
        self.frame_time = frame_time;
    }

    pub fn get_frame_time(&self) -> u128 {
        self.frame_time
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.frame.to_bytes()
    }

    pub fn len(&self) -> usize {
        self.frame.len()
    }

    pub fn compute_length(&mut self) -> usize {
        let length = self.frame.compute_length();
        self.length = length;
        length
    }
}
