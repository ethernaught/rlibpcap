use std::any::Any;
use std::fmt::Debug;
use crate::packet::layers::ethernet_frame::llc::inter::llc_types::LlcTypes;
use crate::packet::layers::inter::layer::Layer;

pub const LLC_EXTENSION_LEN: usize = 3;

#[derive(Clone, Debug)]
pub struct LlcExtension {
    dsap: u8,
    ssap: u8,
    control: LlcTypes, //CHANGE TO ENUM TYPE LATER...
    //snap: Option<>
}

impl LlcExtension {

    pub fn new(dsap: u8, ssap: u8, control: LlcTypes) -> Self {
        Self {
            dsap,
            ssap,
            control
        }
    }

    pub fn set_dsap(&mut self, dsap: u8) {
        self.dsap = dsap;
    }

    pub fn get_dsap(&self) -> u8 {
        self.dsap
    }

    pub fn set_ssap(&mut self, ssap: u8) {
        self.ssap = ssap;
    }

    pub fn get_ssap(&self) -> u8 {
        self.ssap
    }

    pub fn set_control(&mut self, control: LlcTypes) {
        self.control = control;
    }

    pub fn get_control(&self) -> LlcTypes {
        self.control
    }
}

impl Layer for LlcExtension {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < LLC_EXTENSION_LEN {
            return None;
        }

        Some(Self {
            dsap: buf[0],
            ssap: buf[1],
            control: LlcTypes::from_code(buf[2]).unwrap()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; LLC_EXTENSION_LEN];

        buf[0] = self.dsap;
        buf[1] = self.ssap;
        buf[2] = self.control.get_code();

        buf
    }

    fn len(&self) -> usize {
        LLC_EXTENSION_LEN
    }

    fn compute_length(&mut self) -> usize {
        LLC_EXTENSION_LEN
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
