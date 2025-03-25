use std::any::Any;
use crate::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::ethernet_frame::ip::inter::ip_versions::IpVersions;

#[derive(Clone, Debug)]
pub struct RawFrame {
    data: Option<Box<dyn Layer>>,
    length: usize
}

impl RawFrame {

    pub fn new() -> Self {
        Self {
            data: None,
            length: 0
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

impl Layer for RawFrame {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 1 {
            return None;
        }

        let _type = IpVersions::from_code((buf[0] >> 4) & 0x0F).unwrap();

        let data = match _type {
            IpVersions::Ipv4 => {
                Some(Ipv4Layer::from_bytes(buf).unwrap().dyn_clone())
            }
            IpVersions::Ipv6 => {
                Some(Ipv6Layer::from_bytes(buf).unwrap().dyn_clone())
            }
        };

        Some(Self {
            data,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        match &self.data {
            Some(data) => {
                data.to_bytes()
            }
            None => {
                Vec::new()
            }
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn compute_length(&mut self) -> usize {
        self.length = match &self.data {
            Some(layer) => {
                layer.len()
            }
            None => {
                0
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
