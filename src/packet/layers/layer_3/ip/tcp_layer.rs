use std::any::Any;
use crate::packet::layers::inter::layer::Layer;

#[derive(Clone, Debug)]
pub struct TcpLayer {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgment_number: u32,
    data_offset: u8,
    flags: u16,
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16
}

impl TcpLayer {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 20 {
            return None;
        }

        Some(Self {
            source_port: u16::from_be_bytes([buf[0], buf[1]]),
            destination_port: u16::from_be_bytes([buf[2], buf[3]]),
            sequence_number: u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]),
            acknowledgment_number: u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]),
            data_offset: (buf[12] >> 4) * 4,
            flags: u16::from_be_bytes([buf[12] & 0x0F, buf[13]]),
            window_size: u16::from_be_bytes([buf[14], buf[15]]),
            checksum: u16::from_be_bytes([buf[16], buf[17]]),
            urgent_pointer: u16::from_be_bytes([buf[18], buf[19]])
        })
    }

    pub fn get_source_port(&self) -> u16 {
        self.source_port
    }

    pub fn get_destination_port(&self) -> u16 {
        self.destination_port
    }

    pub fn get_sequence_number(&self) -> u32 {
        self.sequence_number
    }

    pub fn get_acknowledgment_number(&self) -> u32 {
        self.acknowledgment_number
    }

    pub fn get_data_offset(&self) -> u8 {
        self.data_offset
    }

    pub fn get_flags(&self) -> u16 {
        self.flags
    }

    pub fn get_window_size(&self) -> u16 {
        self.window_size
    }

    pub fn get_checksum(&self) -> u16 {
        self.checksum
    }

    pub fn get_urgent_pointer(&self) -> u16 {
        self.urgent_pointer
    }
}

impl Layer for TcpLayer {

    fn len(&self) -> usize {
        20
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
