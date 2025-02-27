pub mod devices;
pub mod packet;

/*
[
    Device {
        name: "wlp7s0",
        desc: None,
        addresses: [
            Address {
                addr: 192.168.0.129,
                netmask: Some(255.255.255.0),
                broadcast_addr: Some(192.168.0.255),
                dst_addr: None
            },
            Address {
                addr: xxx:xxx:xxx:xxx,
                netmask: Some(ffff:ffff:ffff:ffff::),
                broadcast_addr: None,
                dst_addr: None
            }
        ],
        flags: DeviceFlags {
            if_flags: UP | RUNNING | WIRELESS, connection_status: Connected
        }
    }
]
*/

#[cfg(target_os = "linux")]
pub mod capture {
    use std::{io, mem};
    use std::os::fd::RawFd;
    use crate::devices::Device;
    use crate::packet::packet::{decode_packet, Packet};

    pub const SYS_SOCKET: i64 = 41;
    pub const AF_PACKET: i64 = 17;
    pub const SOCK_RAW: i64 = 3;
    pub const ETH_P_ALL: u16 = 0x0003;
    pub const SOL_SOCKET: i64 = 1;
    pub const SO_BINDTODEVICE: i64 = 25;
    pub const SYS_IOCTL: i64 = 16;
    pub const SYS_BIND: i64 = 49;
    pub const SYS_RECV_FROM: i64 = 45;
    pub const SYS_SET_SOCK_OPT: i64 = 54;
    pub const IFNAMSIZ: usize = 16;
    pub const SIOCGIFINDEX: u64 = 0x8933;

    #[repr(C)]
    pub struct IfReq {
        ifr_name: [u8; IFNAMSIZ],
        ifr_ifindex: i32,
    }

    #[repr(C)]
    pub struct SockAddrLl {
        sll_family: u16,
        sll_protocol: u16,
        sll_ifindex: i32,
        sll_hatype: u16,
        sll_pkttype: u8,
        sll_halen: u8,
        sll_addr: [u8; 8],
    }

    #[derive(Debug)]
    pub struct Capture {
        fd: RawFd,
        device: Device,
        promiscuous: bool
    }

    impl Capture {

        pub fn from_device(device: &Device) -> io::Result<Self> {
            let fd = unsafe {
                Self::syscall(SYS_SOCKET, AF_PACKET, SOCK_RAW, ETH_P_ALL.to_be() as i64, 0, 0)
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

        pub fn open(&self) -> io::Result<()> {
            if self.fd < 0 {
                return Err(io::Error::last_os_error());
            }

            let res = match !self.promiscuous {
                true => {
                    let mut ifreq = IfReq {
                        ifr_name: [0; IFNAMSIZ],
                        ifr_ifindex: 0,
                    };

                    let if_name_bytes = self.device.get_name().into_bytes();
                    if if_name_bytes.len() >= IFNAMSIZ {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
                    }

                    ifreq.ifr_name[..if_name_bytes.len()].copy_from_slice(&if_name_bytes);

                    let res = unsafe {
                        Self::syscall(SYS_IOCTL, self.fd as i64, SIOCGIFINDEX as i64, &mut ifreq as *mut _ as i64, 0, 0)
                    };

                    if res < 0 {
                        return Err(io::Error::last_os_error());
                    }

                    let if_index = ifreq.ifr_ifindex;

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
                        Self::syscall(SYS_BIND, self.fd as i64, &sockaddr as *const _ as i64, mem::size_of::<SockAddrLl>() as i64, 0, 0)
                    };

                    if res < 0 {
                        return Err(io::Error::last_os_error());
                    }

                    unsafe {
                        Self::syscall(SYS_SET_SOCK_OPT, self.fd as i64, SOL_SOCKET, SO_BINDTODEVICE, ifreq.ifr_name.as_ptr() as i64, IFNAMSIZ as i64)
                    }
                }
                false => {
                    unsafe {
                        Self::syscall(SYS_SET_SOCK_OPT, self.fd as i64, SOL_SOCKET, SO_BINDTODEVICE, 0, 0)
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

        pub fn next_packet(&mut self) -> io::Result<Packet> {
            let mut buffer = vec![0u8; 4096];

            let len = unsafe {
                Self::syscall(
                    SYS_RECV_FROM,
                    self.fd as i64,
                    buffer.as_mut_ptr() as i64,
                    buffer.len() as i64,
                    0,
                    0
                )
            };

            if len > 0 {
                Ok(decode_packet(self.device.get_interface(), &buffer[..len as usize]))

            } else {
                Err(io::Error::last_os_error())
            }
        }

        unsafe fn syscall(number: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
            let ret: i64;
            core::arch::asm!("syscall", in("rax") number, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rax") ret);
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::capture::Capture;
    use crate::devices::Device;

    /*
    update values of dhcp layer
    try not to use vec::new
    frame should calculate length on the spot...

    - Honestly redo all of the packet class
    - IE length, interface, and frame time may not be needed...
    */

    #[test]
    fn test() {
        let devices = Device::list().unwrap();
        println!("{:?}", devices);

        let device = devices.into_iter().find(|d| d.get_name().contains("Ethernet adapter Ethernet:")).unwrap();
        println!("{:?}", device);

        let mut cap = Capture::from_device(&device).unwrap();
        cap.open().unwrap();


        while let Ok(packet) = cap.next_packet() {
            println!("{:x?}", packet);
        }
    }
}
