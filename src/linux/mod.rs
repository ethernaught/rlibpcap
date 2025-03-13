use std::ffi::{c_char, c_int, c_short};
use std::os::fd::RawFd;

pub mod capture;
pub mod devices;

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
const SIOCGIFCONF: u64 = 0x8912;
const SIOCGIFADDR: u64 = 0x8915;
const SIOCGIFFLAGS: u64 = 0x8913;
pub const SIOCGIFHWADDR: u64 = 0x00008927;

pub const AF_INET: i64 = 2;
pub const SOCK_DGRAM: i64 = 2;

#[repr(C)]
pub struct Ifreq2 {
    ifr_name: [u8; IFNAMSIZ], // Interface name (e.g., "eth0")
    ifr_addr: [u8; 24],  // IP address (for IPv4)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Ifreq {
    ifr_name: [u8; IFNAMSIZ],
    ifr_ifru: IfrIfru
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
#[derive(Debug, Copy, Clone)]
struct IfConf {
    ifc_len: c_int,
    ifc_buf: *mut Ifreq2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct IfreqIndex {
    ifr_name: [c_char; IFNAMSIZ],
    ifr_ifindex: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct IfreqFlags {
    ifr_name: [c_char; IFNAMSIZ],
    ifr_flags: c_short,
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

unsafe fn socket(domain: i64, _type: i64, protocol: i64) -> RawFd {
    syscall(SYS_SOCKET, domain, _type, protocol, 0, 0) as RawFd
}

unsafe fn ioctl(fd: RawFd, request: i64, arg: i64) -> i64 {
    syscall(SYS_IOCTL, fd as i64, request, arg, 0, 0)
}

unsafe fn close(fd: RawFd) {

}

unsafe fn syscall(number: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    let ret: i64;
    core::arch::asm!("syscall", in("rax") number, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rax") ret);
    ret
}
