use std::{io, mem, ptr};
use std::ffi::{c_char, c_void};
use std::io::Error;
use std::net::Ipv4Addr;
use crate::packet::packet::Packet;
use crate::windows::devices::Device;
use crate::windows::sys::{bind, recvfrom, SockAddr, socket, WsaData, WSAIoctl, WSAStartup, AF_INET, IPPROTO_IP, RCVALL_ON, SIO_RCVALL, SOCK_RAW};

#[derive(Debug, Clone)]
pub struct Capture {
    //fd: RawFd,
    device: Option<Device>
}

impl Capture {

    pub fn from_device(device: &Device) -> io::Result<Self> {
        Ok(Self {
            device: Some(device.clone())
        })
    }

    pub fn open(&self) -> io::Result<()> {
        unsafe {
            let mut wsa_data: WsaData = unsafe { mem::zeroed() };
            if unsafe { WSAStartup(0x202, &mut wsa_data) } != 0 {
                panic!("WSAStartup failed");
            }

            let sock = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
            if sock == usize::MAX {
                let err = Error::last_os_error();
                panic!("Failed to create raw socket: {}", err);
            }


            // Replace with your local IP address
            let local_ip = Ipv4Addr::new(192, 168, 0, 51);
            let mut addr = SockAddr {
                sa_family: AF_INET as u16,
                sa_data: [0; 14],
            };

            // Fill sa_data with IP address bytes in the right offset
            addr.sa_data[2..6].copy_from_slice(&local_ip.octets());

            if bind(sock, &addr, mem::size_of::<SockAddr>() as i32) != 0 {
                panic!("Failed to bind raw socket: {}", Error::last_os_error());
            }




            let mut bytes_returned: u32 = 0;
            let mut enable: u32 = RCVALL_ON;

            let result = WSAIoctl(sock, SIO_RCVALL, &mut enable as *mut _ as *mut c_void, mem::size_of::<u32>() as u32, ptr::null_mut(), 0, &mut bytes_returned, ptr::null_mut(), None);



            if result != 0 {
                panic!("WSAIoctl SIO_RCVALL failed: {}", Error::last_os_error());
            }

            println!("Listening for incoming IP packets...");

            let mut buf = [0u8; 65535];
            loop {
                let mut from = SockAddr { sa_family: 0, sa_data: [0; 14] };
                let mut fromlen = mem::size_of::<SockAddr>() as i32;

                let len = recvfrom(sock, buf.as_mut_ptr() as *mut c_char, buf.len() as i32, 0, &mut from, &mut fromlen);

                println!("{}", len);

                if len > 0 {
                    println!("Received {} bytes", len);
                    println!("First 16 bytes: {:?}", &buf[..len as usize]);
                }
            }
        }
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

/*
use std::ptr;
use std::mem;
use std::net::Ipv4Addr;
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::io::Error;

type SOCKET = usize;
type DWORD = u32;
type WORD = u16;
type BYTE = u8;

#[repr(C)]
struct WSAData {
    wVersion: WORD,
    wHighVersion: WORD,
    szDescription: [c_char; 257],
    szSystemStatus: [c_char; 129],
    iMaxSockets: WORD,
    iMaxUdpDg: WORD,
    lpVendorInfo: *mut c_char,
}

extern "system" {
    fn WSAStartup(wVersionRequested: WORD, lpWSAData: *mut WSAData) -> c_int;
    fn socket(af: c_int, kind: c_int, protocol: c_int) -> SOCKET;
    fn bind(s: SOCKET, name: *const sockaddr, namelen: c_int) -> c_int;
    fn WSAIoctl(
        s: SOCKET,
        dwIoControlCode: DWORD,
        lpvInBuffer: *mut c_void,
        cbInBuffer: DWORD,
        lpvOutBuffer: *mut c_void,
        cbOutBuffer: DWORD,
        lpcbBytesReturned: *mut DWORD,
        lpOverlapped: *mut c_void,
        lpCompletionRoutine: Option<extern "system" fn()>,
    ) -> c_int;
    fn recvfrom(
        s: SOCKET,
        buf: *mut c_char,
        len: c_int,
        flags: c_int,
        from: *mut sockaddr,
        fromlen: *mut c_int,
    ) -> c_int;
}

const AF_INET: c_int = 2;
const SOCK_RAW: c_int = 3;
const IPPROTO_IP: c_int = 0;
const SIO_RCVALL: DWORD = 0x98000001;
const RCVALL_ON: DWORD = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct sockaddr {
    sa_family: u16,
    sa_data: [u8; 14],
}

fn main() {
    unsafe {
        let mut wsa_data: WSAData = mem::zeroed();
        if WSAStartup(0x202, &mut wsa_data) != 0 {
            panic!("WSAStartup failed");
        }

        let sock = socket(AF_INET, SOCK_RAW, IPPROTO_IP);
        if sock == usize::MAX {
            let err = Error::last_os_error();
            panic!("Failed to create raw socket: {}", err);
        }


        // Replace with your local IP address
        let local_ip = Ipv4Addr::new(192, 168, 0, 51);
        let mut addr = sockaddr {
            sa_family: AF_INET as u16,
            sa_data: [0; 14],
        };

        // Fill sa_data with IP address bytes in the right offset
        addr.sa_data[2..6].copy_from_slice(&local_ip.octets());

        if bind(sock, &addr, mem::size_of::<sockaddr>() as i32) != 0 {
            panic!("Failed to bind raw socket: {}", Error::last_os_error());
        }




        let mut bytes_returned: DWORD = 0;
        let mut enable: DWORD = RCVALL_ON;

        let result = WSAIoctl(
            sock,
            SIO_RCVALL,
            &mut enable as *mut _ as *mut c_void,
            mem::size_of::<DWORD>() as DWORD,
            ptr::null_mut(),
            0,
            &mut bytes_returned,
            ptr::null_mut(),
            None,
        );



        if result != 0 {
            panic!("WSAIoctl SIO_RCVALL failed: {}", Error::last_os_error());
        }

        println!("Listening for incoming IP packets...");

        let mut buf = [0u8; 65535];
        loop {
            let mut from = sockaddr { sa_family: 0, sa_data: [0; 14] };
            let mut fromlen = mem::size_of::<sockaddr>() as i32;

            let len = recvfrom(
                sock,
                buf.as_mut_ptr() as *mut c_char,
                buf.len() as i32,
                0,
                &mut from,
                &mut fromlen,
            );

            println!("{}", len);

            if len > 0 {
                println!("Received {} bytes", len);
                println!("First 16 bytes: {:?}", &buf[..len as usize]);
            }
        }
    }
}
*/
