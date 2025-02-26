use std::net::Ipv4Addr;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use crate::packet::layers::ethernet_frame::inter::types::Types;
use crate::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;

#[derive(Debug, Clone)]
pub struct Packet {
    interface: Interfaces,
    frame: Box<dyn Layer>,
    //layers: Vec<Box<dyn Layer>>,
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
            //layers: Vec::new(),
            frame_time,
            length: data.len()
        }
    }

    pub fn get_interface(&self) -> &Interfaces {
        &self.interface
    }

    pub fn get_frame(&self) -> &Box<dyn Layer> {
        &self.frame
    }

    pub fn get_frame_time(&self) -> u128 {
        self.frame_time
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.frame.to_bytes()
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

pub fn decode_packet(interface: Interfaces, data: &[u8]) -> Packet {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    Packet::new(interface, now, data)
}
