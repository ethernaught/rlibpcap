use std::{io, mem};
use std::os::fd::RawFd;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::devices::Device;
use crate::macos::sys::SockAddrLl;
use crate::packet::inter::data_link_types::DataLinkTypes;
use crate::packet::packet::Packet;

#[derive(Debug, Clone)]
pub struct Capture {
    fd: RawFd,
    device: Option<Device>
}

impl Capture {

    pub fn any() -> io::Result<Self> {
        todo!()
    }

    pub fn from_device(device: &Device) -> io::Result<Self> {
        todo!()
    }

    pub fn open(&self) -> io::Result<()> {

        todo!()
    }

    pub fn set_immediate_mode(&self, immediate: bool) {
        println!("Setting capture to immediate mode");
    }

    pub fn send(&self, packet: Packet) -> io::Result<usize> {
        let mut packet = packet.to_bytes();

        todo!()
    }

    pub fn recv(&self) -> io::Result<(SockAddrLl, Packet)> { //i32 should be the socket address
        self.recv_with_flags(0)
    }

    pub fn try_recv(&self) -> io::Result<(SockAddrLl, Packet)> { //i32 should be the socket address
        self.recv_with_flags(0) //0 SHOULD BE RECEIVE ALL FLAG
    }

    fn recv_with_flags(&self, flags: i64) -> io::Result<(SockAddrLl, Packet)> { //i32 should be the socket address
        todo!()
    }

    pub fn close(&self) {
    }
}
