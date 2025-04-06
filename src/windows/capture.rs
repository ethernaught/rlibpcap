use std::{io, mem};
use crate::packet::packet::Packet;
use crate::windows::devices::Device;

#[derive(Debug, Clone)]
pub struct Capture {
    //fd: RawFd,
    device: Option<Device>
}

impl Capture {


    pub fn any() -> io::Result<Self> {

        Ok(Self {
            device: None
        })
    }

    pub fn from_device(device: &Device) -> io::Result<Self> {

        Ok(Self {
            device: Some(device.clone())
        })
    }

    pub fn open(&self) -> io::Result<()> {

        Ok(())
    }

    pub fn set_immediate_mode(&self, immediate: bool) ->  io::Result<()> {
        Ok(())
    }

    pub fn send(&self, packet: Packet) -> io::Result<usize> {
        todo!()
    }

    pub fn recv(&self) -> io::Result<(i32, Packet)> {
        todo!()
    }

    pub fn try_recv(&self) -> io::Result<(i32, Packet)> {
        todo!()
    }

    fn recv_with_flags(&self, flags: i64) -> io::Result<(i32, Packet)> {
        todo!()
    }

    pub fn close(&self) {
        //unsafe { close(self.fd) };
    }

    pub fn get_device(&self) -> Option<&Device> {
        self.device.as_ref()
    }
}
