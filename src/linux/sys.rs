use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::os::fd::RawFd;

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
pub const SIOCGIFCONF: u64 = 0x8912;
pub const SIOCGIFADDR: u64 = 0x8915;
pub const SIOCGIFFLAGS: u64 = 0x8913;
pub const SIOCGIFHWADDR: u64 = 0x00008927;

pub const AF_INET: i64 = 2;
pub const AF_INET6: i64 = 10;
pub const SOCK_DGRAM: i64 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ifreq {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_ifru: IfrIfru
}

#[repr(C)]
#[derive(Copy, Clone)]
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
pub struct IfreqAddr {
    pub ifr_name: [u8; IFNAMSIZ], // Interface name (e.g., "eth0")
    pub ifr_addr: [u8; 24],  // IP address (for IPv4)
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfConf {
    pub ifc_len: i32,
    pub ifc_buf: *mut IfreqAddr,
}
/*
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct IfreqIndex {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_ifindex: i32,
}
*/
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IfreqFlags {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_flags: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SockAddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
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
/*
const IFF_UP: u16 = 0x1;
const IFF_BROADCAST: u16 = 0x2;
const IFF_DEBUG: u16 = 0x4;
const IFF_LOOPBACK: u16 = 0x8;
const IFF_POINTOPOINT: u16 = 0x10;
const IFF_NOTRAILERS: u16 = 0x20;
const IFF_RUNNING: u16 = 0x40;
const IFF_NOARP: u16 = 0x80;
const IFF_PROMISC: u16 = 0x100;
const IFF_ALLMULTI: u16 = 0x200;
const IFF_MASTER: u16 = 0x400;
const IFF_SLAVE: u16 = 0x800;
const IFF_MULTICAST: u16 = 0x1000;
const IFF_PORTSEL: u16 = 0x2000;
const IFF_AUTOMEDIA: u16 = 0x4000;
const IFF_DYNAMIC: u16 = 0x8000;

fn print_flags(flags: u16) {
    println!("Interface Flags: {:04x}", flags);

    if flags & IFF_UP != 0 { println!(" - UP"); }
    if flags & IFF_BROADCAST != 0 { println!(" - BROADCAST"); }
    if flags & IFF_DEBUG != 0 { println!(" - DEBUG"); }
    if flags & IFF_LOOPBACK != 0 { println!(" - LOOPBACK"); }
    if flags & IFF_POINTOPOINT != 0 { println!(" - POINTOPOINT"); }
    if flags & IFF_NOTRAILERS != 0 { println!(" - NOTRAILERS"); }
    if flags & IFF_RUNNING != 0 { println!(" - RUNNING"); }
    if flags & IFF_NOARP != 0 { println!(" - NOARP"); }
    if flags & IFF_PROMISC != 0 { println!(" - PROMISCUOUS"); }
    if flags & IFF_ALLMULTI != 0 { println!(" - ALLMULTI"); }
    if flags & IFF_MASTER != 0 { println!(" - MASTER"); }
    if flags & IFF_SLAVE != 0 { println!(" - SLAVE"); }
    if flags & IFF_MULTICAST != 0 { println!(" - MULTICAST"); }
    if flags & IFF_PORTSEL != 0 { println!(" - PORTSEL"); }
    if flags & IFF_AUTOMEDIA != 0 { println!(" - AUTOMEDIA"); }
    if flags & IFF_DYNAMIC != 0 { println!(" - DYNAMIC"); }
}
*/
