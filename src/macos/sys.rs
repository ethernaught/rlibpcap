use std::arch::asm;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::os::fd::RawFd;

pub const SYS_SOCKET: i64 = 0x2000061; // 97
pub const SYS_CLOSE: i64 = 0x2000006;  // 6
pub const SYS_IOCTL: i64 = 0x2000062;  // 98
pub const SYS_BIND: i64 = 0x2000068;   // 104
pub const SYS_SENDTO: i64 = 0x200006e; // 110


pub const SYS_FCNTL: i64 = 0x200005c; // Correct value for `fcntl` on macOS



pub const IFNAMSIZ: usize = 16;

pub const SIOCGIFCONF: u64 = 0xc0086914;




pub const AF_INET: i64 = 2;
pub const SOCK_DGRAM: i64 = 2;



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfreqName {
    pub ifr_name: [u8; IFNAMSIZ]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfreqAddr {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_addr: [u8; 16]
}

/*
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfreqHwAddr {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_hwaddr: SockAddr
}
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfConf {
    pub ifc_len: i32,
    pub ifc_buf: *mut IfreqAddr,
}








#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SockAddrLl {
    pub sll_family: u16,
    pub sll_protocol: u16,
    pub sll_ifindex: i32,
    pub sll_hatype: u16,
    pub sll_pkttype: u8,
    pub sll_halen: u8,
    pub sll_addr: [u8; 8]
}






pub unsafe fn socket(domain: i64, _type: i64, protocol: i64) -> RawFd {
    syscall(SYS_SOCKET, domain, _type, protocol, 0, 0) as RawFd
}

pub unsafe fn bind(fd: RawFd, address: i64, address_len: i64) -> i64 {
    syscall(SYS_BIND, fd as i64, address, address_len, 0, 0)
}

pub unsafe fn ioctl(fd: RawFd, request: i64, arg: i64) -> i64 {
    //syscall(SYS_IOCTL, fd as i64, request, arg, 0, 0)
    let ret: i64;
    asm!("svc 0", in("x8") SYS_IOCTL, in("x0") fd as i64, in("x1") request, in("x2") arg, lateout("x0") ret);
    ret
}

pub unsafe fn sendto(fd: RawFd, buffer: &mut [u8]) -> i64 {
    syscall(SYS_SENDTO, fd as i64, buffer.as_mut_ptr() as *mut _ as i64, buffer.len() as i64, 0, 0)
}

pub unsafe fn close(fd: RawFd) {
    syscall(SYS_CLOSE, fd as i64, 0, 0, 0, 0);
}

//for APPLE CHIPS
pub unsafe fn syscall(number: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    let ret: i64;
    asm!("svc 0", in("x8") number, in("x0") a1, in("x1") a2, in("x2") a3, in("x3") a4, in("x4") a5, lateout("x0") ret);
    ret
}

pub fn parse_ip(buf: &[u8]) -> Option<IpAddr> {
    if buf.len() < 5 {
        return None;
    }

    match buf[0] as i64 {
        AF_INET => {
            if buf.len() < 8 {
                return None;
            }
            Some(IpAddr::V4(Ipv4Addr::new(buf[4], buf[5], buf[6], buf[7])))
        }
        AF_INET6 => {
            if buf.len() < 20 {
                return None;
            }
            Some(IpAddr::V6(Ipv6Addr::from([
                buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11],
                buf[12], buf[13], buf[14], buf[15], buf[16], buf[17], buf[18], buf[19]
            ])))
        }
        _ => None
    }
}
