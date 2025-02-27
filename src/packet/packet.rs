use std::time::{SystemTime, UNIX_EPOCH};
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use crate::packet::layers::inter::layer::Layer;

#[derive(Debug, Clone)]
pub struct Packet {
    interface: Interfaces,
    frame: Box<dyn Layer>,
    frame_time: u128,
    length: usize
}

impl Packet {

    pub fn new(interface: Interfaces, frame_time: u128, data: &[u8]) -> Self {
        let frame = match interface {
            Interfaces::Ethernet => {
                EthernetFrame::from_bytes(data).unwrap().dyn_clone()
            }
            Interfaces::WiFi => {
                todo!()
            }
            Interfaces::Bluetooth => {
                todo!()
            }
        };

        Self {
            interface,
            frame,
            frame_time,
            length: data.len()
        }
    }

    pub fn set_interface(&mut self, interface: Interfaces) {
        self.interface = interface;
    }

    pub fn get_interface(&self) -> &Interfaces {
        &self.interface
    }

    pub fn set_frame(&mut self, frame: Box<dyn Layer>) {
        self.length = frame.len();
        self.frame = frame;
    }

    pub fn get_frame(&self) -> &Box<dyn Layer> {
        &self.frame
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

pub fn decode_packet(interface: Interfaces, data: &[u8]) -> Packet {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    Packet::new(interface, now, data)
}
