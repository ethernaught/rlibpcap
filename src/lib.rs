extern crate core;

pub mod capture;
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

#[cfg(test)]
mod tests {
    use core::asserting::Capture;
    use crate::devices::Device;

    #[test]
    fn it_works() {
        let devices = Device::list().unwrap();
        println!("{:?}", devices);

        let device = devices.into_iter().find(|d| d.get_name().contains("Ethernet adapter Ethernet:")).unwrap();
        println!("{:?}", device);

        let mut cap = Capture::from_device(device).unwrap();
        cap.open().unwrap();


        while let Ok(packet) = cap.next_packet() {
            println!("{:x?}", packet);
        }
    }
}

#[cfg(target_os = "linux")]
pub mod capture {
    use std::io;
    use std::os::fd::RawFd;
    use crate::devices::Device;
    use crate::packet::packet::{decode_packet, Packet};

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

        pub fn set_immediate_mode(&self, immediate: bool) {
            println!("Setting immediate mode for interface {}", self.device.get_name());
        }

        pub fn set_promiscuous_mode(&self, promiscuous: bool) {
            println!("Setting promiscuous mode for interface {}", self.device.get_name());
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
            } as u32;

            if len > 0 {
                Ok(decode_packet(self.device.get_interface(), &buffer[..len as usize], len))

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
}
