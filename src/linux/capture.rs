use std::{io, mem};
use std::os::fd::RawFd;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::devices::Device;
use crate::linux::sys::{bind, close, recvfrom, sendto, setsockopt, socket, syscall, IfreqName, MSG_DONTWAIT};
use crate::packet::packet::Packet;
use crate::linux::sys::{SockAddrLl, AF_PACKET, ETH_P_ALL, IFNAMSIZ, SOCK_RAW, SOL_SOCKET, SO_BINDTODEVICE, SYS_BIND, SYS_RECV_FROM, SYS_SENDTO, SYS_SET_SOCK_OPT, SYS_SOCKET};
use crate::utils::data_link_types::DataLinkTypes;

#[derive(Debug, Clone)]
pub struct Capture {
    fd: RawFd,
    device: Option<Device>
}

impl Capture {

    pub fn any() -> io::Result<Self> {
        let fd = unsafe { socket(AF_PACKET, SOCK_RAW, ETH_P_ALL.to_be() as i64) };

        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd: fd as RawFd,
            device: None
        })
    }

    pub fn from_device(device: &Device) -> io::Result<Self> {
        let fd = unsafe { socket(AF_PACKET, SOCK_RAW, ETH_P_ALL.to_be() as i64) };

        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd: fd as RawFd,
            device: Some(device.clone())
        })
    }

    pub fn open(&self) -> io::Result<()> {
        if self.fd < 0 {
            return Err(io::Error::last_os_error());
        }

        if match self.device {
            Some(ref device) => {
                let mut ifreq = IfreqName {
                    ifr_name: [0; IFNAMSIZ],
                };

                let if_name_bytes = device.get_name().into_bytes();
                if if_name_bytes.len() >= IFNAMSIZ {
                    unsafe { close(self.fd) };
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
                }

                ifreq.ifr_name[..if_name_bytes.len()].copy_from_slice(&if_name_bytes);

                let sockaddr = SockAddrLl {
                    sll_family: AF_PACKET as u16,
                    sll_protocol: ETH_P_ALL.to_be(),
                    sll_ifindex: device.get_index(),
                    sll_hatype: 0,
                    sll_pkttype: 0,
                    sll_halen: 0,
                    sll_addr: [0; 8],
                };

                if unsafe { bind(self.fd, &sockaddr as *const _ as i64, mem::size_of::<SockAddrLl>() as i64) } < 0 {
                    unsafe { close(self.fd) };
                    return Err(io::Error::last_os_error());
                }

                unsafe {
                    setsockopt(self.fd, SOL_SOCKET, SO_BINDTODEVICE, ifreq.ifr_name.as_ptr() as i64, IFNAMSIZ as i64)
                }
            }
            None => {
                unsafe {
                    setsockopt(self.fd, SOL_SOCKET, SO_BINDTODEVICE, 0, 0)
                }
            }
        } < 0 {
            unsafe { close(self.fd) };
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn set_immediate_mode(&self, immediate: bool) ->  io::Result<()> {
        Ok(())
    }

    pub fn send(&self, packet: Packet) -> io::Result<usize> {
        let mut packet = packet.to_bytes();

        let len = unsafe { sendto(self.fd, &mut packet) };

        if len > 0 {
            Ok(len as usize)
        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn recv(&self) -> io::Result<(SockAddrLl, Packet)> {
        self.recv_with_flags(0)
    }

    pub fn try_recv(&self) -> io::Result<(SockAddrLl, Packet)> {
        self.recv_with_flags(MSG_DONTWAIT)
    }

    fn recv_with_flags(&self, flags: i64) -> io::Result<(SockAddrLl, Packet)> {
        let mut buffer = vec![0u8; 4096];
        let mut sockaddr: SockAddrLl = unsafe { mem::zeroed() };

        let len = unsafe { recvfrom(self.fd, &mut buffer, flags, &mut sockaddr) };

        if len > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            let data_link_type = DataLinkTypes::from_sll2_code(sockaddr.sll_hatype)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            return Ok((sockaddr, Packet::new(data_link_type, now, &buffer[..len as usize])));

        } else if len == -1 {
            return Err(io::Error::last_os_error());
        }

        Err(io::Error::new(io::ErrorKind::WouldBlock, "No data available"))
    }

    pub fn close(&self) {
        unsafe { close(self.fd) };
    }

    pub fn get_device(&self) -> Option<&Device> {
        self.device.as_ref()
    }
}
