use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::inter::layer::Layer;

#[derive(Debug)]
pub struct Packet {
    interface: Interfaces,
    layers: Vec<Box<dyn Layer>>,
    frame_time: u128,
    length: u32
}

impl Packet {

    pub fn new(interface: Interfaces, frame_time: u128, length: u32) -> Self {
        Self {
            interface,
            layers: Vec::new(),
            frame_time,
            length
        }
    }

    pub fn get_interface(&self) -> &Interfaces {
        &self.interface
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn get_layer(&self, index: usize) -> Option<&Box<dyn Layer>> {
        self.layers.get(index)
    }

    pub fn get_layers(&self) -> &Vec<Box<dyn Layer>> {
        &self.layers
    }

    pub fn get_total_layers(&self) -> usize {
        self.layers.len()
    }

    pub fn get_frame_time(&self) -> u128 {
        self.frame_time
    }

    pub fn len(&self) -> u32 {
        self.length
    }
}
