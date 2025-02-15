use std::io;
use std::os::fd::RawFd;
use crate::devices::Device;

const AF_PACKET: i64 = 17;
const SOCK_RAW: i64 = 3;
const ETH_P_ALL: u16 = 0x0003;
const SOL_SOCKET: i64 = 1;
const SO_BINDTODEVICE: i64 = 25;

#[derive(Debug)]
pub struct Capture {
    fd: RawFd,
    device: Device
}

#[cfg(target_os = "linux")]
impl Capture {

    pub fn from_device(device: Device) -> io::Result<Self> {
        let fd = unsafe {
            Self::syscall(41, AF_PACKET, SOCK_RAW, ETH_P_ALL.to_be() as i64, 0, 0)
        };

        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd: fd as RawFd,
            device
        })
    }

    pub fn open(&self) -> io::Result<()> {
        if self.fd < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut if_name_bytes = [0u8; 16];
        if let bytes = self.device.get_name().as_bytes() {
            if bytes.len() >= if_name_bytes.len() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
            }

            if_name_bytes[..bytes.len()].copy_from_slice(bytes);
        }

        let res = unsafe {
            Self::syscall(
                54,
                self.fd as i64,
                SOL_SOCKET,
                SO_BINDTODEVICE,
                if_name_bytes.as_ptr() as i64,
                if_name_bytes.len() as i64
            )
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn set_immediate_mode(&self, immediate: bool) -> io::Result<()> {
        println!("Setting immediate mode for interface {}", self.device.get_name());
        Ok(())
    }

    pub fn set_promiscuous_mode(&self, promiscuous: bool) -> io::Result<()> {
        println!("Setting promiscuous mode for interface {}", self.device.get_name());
        Ok(())
    }

    pub fn next_packet(&mut self) -> io::Result<Packet> {
        let mut buffer = vec![0u8; 4096];

        let len = unsafe {
            Self::syscall(
                45,
                self.fd as i64,
                buffer.as_mut_ptr() as i64,
                buffer.len() as i64,
                0,
                0
            )
        };

        if len > 0 {
            Ok(Packet::new(buffer[..len as usize].to_vec()))

        } else {
            Err(io::Error::last_os_error())
        }
    }

    unsafe fn syscall(num: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
        let ret: i64;
        core::arch::asm!("syscall", in("rax") num, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rax") ret);
        ret
    }
}

#[derive(Debug)]
pub struct Packet {
    data: Vec<u8>
}

impl Packet {

    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
