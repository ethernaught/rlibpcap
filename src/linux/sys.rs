use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::os::fd::RawFd;

pub const SYS_SOCKET: i64 = 41;
pub const AF_PACKET: i64 = 17;
pub const SOCK_RAW: i64 = 3;
pub const ETH_P_ALL: u16 = 0x0003;
pub const SOL_SOCKET: i64 = 1;
pub const SYS_CLOSE: i64 = 3;
pub const SYS_IOCTL: i64 = 16;
pub const SYS_BIND: i64 = 49;
pub const SYS_SENDTO: i64 = 0x2C;
pub const SYS_RECV_FROM: i64 = 45;
pub const SYS_SET_SOCK_OPT: i64 = 54;
pub const SO_BINDTODEVICE: i64 = 25;
pub const IFNAMSIZ: usize = 16;
pub const SIOCGIFINDEX: u64 = 0x8933;
pub const SIOCGIFCONF: u64 = 0x8912;
pub const SIOCGIFADDR: u64 = 0x8915;
pub const SIOCGIFFLAGS: u64 = 0x8913;
pub const SIOCGIFHWADDR: u64 = 0x00008927;

pub const AF_INET: i64 = 2;
pub const AF_INET6: i64 = 10;
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
    pub ifr_addr: [u8; 24]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfreqHwAddr {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_hwaddr: SockAddr
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfConf {
    pub ifc_len: i32,
    pub ifc_buf: *mut IfreqAddr,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfreqIndex {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_ifindex: i32
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfreqFlags {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_flags: i16
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SockAddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14]
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

pub unsafe fn ioctl(fd: RawFd, request: i64, arg: i64) -> i64 {
    syscall(SYS_IOCTL, fd as i64, request, arg, 0, 0)
}

pub unsafe fn close(fd: RawFd) {
    unsafe { syscall(SYS_CLOSE, fd as i64, 0, 0, 0, 0) };
}

pub unsafe fn syscall(number: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    let ret: i64;
    core::arch::asm!("syscall", in("rax") number, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rax") ret);
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
