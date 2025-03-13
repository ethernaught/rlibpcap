use std::{io, mem};
use std::os::fd::RawFd;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::devices::Device;
use crate::linux::sys::{close, syscall, IfreqInterface};
use crate::packet::packet::Packet;
use crate::linux::sys::{SockAddrLl, AF_PACKET, ETH_P_ALL, IFNAMSIZ, SOCK_RAW, SOL_SOCKET, SO_BINDTODEVICE, SYS_BIND, SYS_RECV_FROM, SYS_SENDTO, SYS_SET_SOCK_OPT, SYS_SOCKET};

#[derive(Debug, Clone)]
pub struct Capture {
    fd: RawFd,
    device: Device,
    promiscuous: bool
}

impl Capture {

    pub fn from_device(device: &Device) -> io::Result<Self> {
        let fd = unsafe {
            syscall(SYS_SOCKET, AF_PACKET, SOCK_RAW, ETH_P_ALL.to_be() as i64, 0, 0)
        };

        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd: fd as RawFd,
            device: device.clone(),
            promiscuous: false
        })
    }

    pub fn open(&mut self) -> io::Result<()> {
        if self.fd < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut ifreq = IfreqInterface {
            ifr_name: [0; IFNAMSIZ],
        };

        let if_name_bytes = self.device.get_name().into_bytes();
        if if_name_bytes.len() >= IFNAMSIZ {
            unsafe { close(self.fd) };
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
        }

        ifreq.ifr_name[..if_name_bytes.len()].copy_from_slice(&if_name_bytes);

        let res = match !self.promiscuous {
            true => {
                let sockaddr = SockAddrLl {
                    sll_family: AF_PACKET as u16,
                    sll_protocol: ETH_P_ALL.to_be(),
                    sll_ifindex: self.device.get_index(),
                    sll_hatype: 0,
                    sll_pkttype: 0,
                    sll_halen: 0,
                    sll_addr: [0; 8],
                };

                let res = unsafe {
                    syscall(SYS_BIND, self.fd as i64, &sockaddr as *const _ as i64, mem::size_of::<SockAddrLl>() as i64, 0, 0)
                };

                if res < 0 {
                    unsafe { close(self.fd) };
                    return Err(io::Error::last_os_error());
                }

                unsafe {
                    syscall(SYS_SET_SOCK_OPT, self.fd as i64, SOL_SOCKET, SO_BINDTODEVICE, ifreq.ifr_name.as_ptr() as i64, IFNAMSIZ as i64)
                }
            }
            false => {
                unsafe {
                    syscall(SYS_SET_SOCK_OPT, self.fd as i64, SOL_SOCKET, SO_BINDTODEVICE, 0, 0)
                }
            }
        };

        if res < 0 {
            unsafe { close(self.fd) };
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn set_immediate_mode(&self, immediate: bool) {
        println!("Setting immediate mode for interface {}", self.device.get_name());
    }

    pub fn set_promiscuous_mode(&mut self, promiscuous: bool) -> io::Result<()> {
        if self.fd < 0 {
            return Err(io::Error::last_os_error());
        }

        self.promiscuous = promiscuous;
        Ok(())
    }

    pub fn send_packet(&self, packet: Packet) -> io::Result<usize> {
        let packet = packet.to_bytes();

        let len = unsafe {
            syscall(SYS_SENDTO, self.fd as i64, packet.as_ptr() as i64, packet.len() as i64, 0, 0)
        };

        if len > 0 {
            Ok(len as usize)
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn next_packet(&mut self) -> io::Result<Packet> {
        let mut buffer = vec![0u8; 4096];

        let len = unsafe {
            syscall(SYS_RECV_FROM, self.fd as i64, buffer.as_mut_ptr() as i64, buffer.len() as i64, 0, 0)
        };

        if len > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            Ok(Packet::new(self.device.get_data_link_type(), now, &buffer[..len as usize]))

        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn close(&self) {
        unsafe { close(self.fd) };
    }
}
