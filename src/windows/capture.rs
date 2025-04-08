use std::{io, mem, ptr};
use std::net::{IpAddr, Ipv4Addr};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::packet::packet::Packet;
use crate::utils::data_link_types::DataLinkTypes;
use crate::windows::devices::Device;
use crate::windows::sys::{bind, recvfrom, SockAddr, socket, WsaData, WSAIoctl, WSAStartup, AF_INET, IPPROTO_IP, RCVALL_ON, SIO_RCVALL, SOCK_RAW, SockAddrIn};

#[derive(Debug, Clone)]
pub struct Capture {
    fd: usize,
    device: Option<Device>
}

impl Capture {

    pub fn from_device(device: &Device) -> io::Result<Self> {
        let mut wsa_data: WsaData = unsafe { mem::zeroed() };
        if unsafe { WSAStartup(0x202, &mut wsa_data) } != 0 {
            return Err(io::Error::last_os_error());
        }

        let fd = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_IP) };
        if fd == usize::MAX {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd,
            device: Some(device.clone())
        })
    }

    pub fn open(&self) -> io::Result<()> {
        match self.device {
            Some(ref device) => {
                match device.get_address() {
                    Some(address) => {
                        let mut addr = SockAddr {
                            sa_family: AF_INET as u16,
                            sa_data: [0; 14]
                        };

                        match address {
                            IpAddr::V4(address) => {
                                addr.sa_data[2..6].copy_from_slice(&address.octets())
                            }
                            IpAddr::V6(_) => {
                                return Err(io::Error::new(io::ErrorKind::Other, "IPv6 is not supported"));
                            }
                        }

                        if unsafe { bind(self.fd, &addr, mem::size_of::<SockAddr>() as i32) } != 0 {
                            return Err(io::Error::last_os_error());
                        }

                        let mut bytes_returned: u32 = 0;
                        let mut enable: u32 = RCVALL_ON;

                        let res = unsafe { WSAIoctl(self.fd, SIO_RCVALL, &mut enable as *mut _ as *mut u16, mem::size_of::<u32>() as u32, ptr::null_mut(), 0, &mut bytes_returned, ptr::null_mut(), None) };

                        if res != 0 {
                            return Err(io::Error::last_os_error());
                        }
                        Ok(())
                    }
                    None => Err(io::Error::new(io::ErrorKind::Other, "Device address was not found"))
                }
            }
            None => Err(io::Error::new(io::ErrorKind::Other, "Failed to open device"))
        }
    }

    pub fn set_immediate_mode(&self, immediate: bool) ->  io::Result<()> {
        Ok(())
    }

    pub fn send(&self, packet: Packet) -> io::Result<usize> {
        todo!()
    }

    pub fn recv(&self) -> io::Result<(i32, Packet)> {
        self.recv_with_flags(0)
    }

    pub fn try_recv(&self) -> io::Result<(i32, Packet)> {
        self.recv_with_flags(0)
    }

    fn recv_with_flags(&self, flags: i64) -> io::Result<(i32, Packet)> {
        let mut buffer = [0u8; 65535];

        let mut from = SockAddrIn {
            sa_family: 0,
            sin_addr: [0u8; 4],
            sin_zero: [0u8; 8]
        };
        let mut fromlen = mem::size_of::<SockAddrIn>() as i32;

        let len = unsafe { recvfrom(self.fd, buffer.as_mut_ptr() as *mut i8, buffer.len() as i32, 0, &mut from, &mut fromlen) };

        if len > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();

            return Ok((0, Packet::new(DataLinkTypes::Raw, now, &buffer[..len as usize])));

        } else if len == -1 {
            return Err(io::Error::last_os_error());
        }

        Err(io::Error::new(io::ErrorKind::WouldBlock, "No data available"))
    }

    pub fn close(&self) {
        //unsafe { close(self.fd) };
    }

    pub fn get_device(&self) -> Option<&Device> {
        self.device.as_ref()
    }
}
