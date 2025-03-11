use std::{io, mem};
use std::os::fd::RawFd;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::devices::Device;
use crate::packet::inter::data_link_types::DataLinkTypes;
use crate::packet::packet::Packet;

pub const SYS_SOCKET: i64 = 41;
pub const AF_PACKET: i64 = 17;
pub const SOCK_RAW: i64 = 3;
pub const ETH_P_ALL: u16 = 0x0003;
pub const SOL_SOCKET: i64 = 1;
pub const SO_BINDTODEVICE: i64 = 25;
pub const SYS_IOCTL: i64 = 16;
pub const SYS_BIND: i64 = 49;
pub const SYS_SENDTO: i64 = 0x2C;
pub const SYS_RECV_FROM: i64 = 45;
pub const SYS_SET_SOCK_OPT: i64 = 54;
pub const IFNAMSIZ: usize = 16;
pub const SIOCGIFINDEX: u64 = 0x8933;
pub const SIOCGIFHWADDR: u64 = 0x00008927;

#[repr(C)]
pub struct IfReq {
    ifr_name: [u8; IFNAMSIZ],
    ifr_ifru: IfrIfru
}

#[repr(C)]
pub union IfrIfru {
    pub ifru_addr: SockAddr,
    pub ifru_dstaddr: SockAddr,
    pub ifru_broadaddr: SockAddr,
    pub ifru_netmask: SockAddr,
    pub ifru_hwaddr: SockAddr,
    pub ifru_flags: i16,
    pub ifru_ifindex: i32,
    pub ifru_metric: i32,
    pub ifru_mtu: i32,
    pub ifru_map: u16,
    pub ifru_slave: [i8; 16],
    pub ifru_newname: [i8; 16],
    pub ifru_data: *mut i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SockAddr {
    pub sa_family: u16,
    pub sa_data: [i8; 14],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SockAddrLl {
    sll_family: u16,
    sll_protocol: u16,
    sll_ifindex: i32,
    sll_hatype: u16,
    sll_pkttype: u8,
    sll_halen: u8,
    sll_addr: [u8; 8]
}

#[derive(Debug, Clone)]
pub struct Capture {
    fd: RawFd,
    device: Device,
    data_link_type: Option<DataLinkTypes>,
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
            data_link_type: None,
            promiscuous: false
        })
    }

    pub fn open(&mut self) -> io::Result<()> {
        if self.fd < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut ifreq: IfReq = unsafe { mem::zeroed() };

        let if_name_bytes = self.device.get_name().into_bytes();
        if if_name_bytes.len() >= IFNAMSIZ {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
        }

        ifreq.ifr_name[..if_name_bytes.len()].copy_from_slice(&if_name_bytes);

        let res = unsafe {
            syscall(SYS_IOCTL, self.fd as i64, SIOCGIFINDEX as i64, &mut ifreq as *mut _ as i64, 0, 0)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        let if_index = unsafe { ifreq.ifr_ifru.ifru_ifindex };

        let res = unsafe {
            syscall(SYS_IOCTL, self.fd as i64, SIOCGIFHWADDR as i64, &mut ifreq as *mut _ as i64, 0, 0)
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        self.data_link_type = Some(unsafe { DataLinkTypes::from_code(ifreq.ifr_ifru.ifru_hwaddr.sa_family as u32)
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e.as_str()))? });

        let res = match !self.promiscuous {
            true => {
                let sockaddr = SockAddrLl {
                    sll_family: AF_PACKET as u16,
                    sll_protocol: ETH_P_ALL.to_be(),
                    sll_ifindex: if_index,
                    sll_hatype: 0,
                    sll_pkttype: 0,
                    sll_halen: 0,
                    sll_addr: [0; 8],
                };

                let res = unsafe {
                    syscall(SYS_BIND, self.fd as i64, &sockaddr as *const _ as i64, mem::size_of::<SockAddrLl>() as i64, 0, 0)
                };

                if res < 0 {
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
            syscall(
                SYS_SENDTO,                  // Send syscall number
                self.fd as i64,              // File descriptor
                packet.as_ptr() as i64,      // Pointer to the data to send
                packet.len() as i64,         // Length of the data
                0,                            // Flags (0 if no flags needed)
                0                             // Address (0 for no address, required for UDP etc.)
            )
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
            syscall(
                SYS_RECV_FROM,
                self.fd as i64,
                buffer.as_mut_ptr() as i64,
                buffer.len() as i64,
                0,
                0
            )
        };

        if len > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            Ok(Packet::new(self.data_link_type.expect("Unknown data link type"), now, &buffer[..len as usize]))

        } else {
            Err(io::Error::last_os_error())
        }
    }

    pub fn get_data_link_type(&self) -> Option<DataLinkTypes> {
        self.data_link_type
    }
}

unsafe fn syscall(number: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    let ret: i64;
    core::arch::asm!("syscall", in("rax") number, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rax") ret);
    ret
}
