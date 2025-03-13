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
pub const SIOCGIFHWADDR: u64 = 0x00008927;

pub const AF_INET: i64 = 2;
pub const SOCK_DGRAM: i64 = 2;

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

unsafe fn syscall(number: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    let ret: i64;
    core::arch::asm!("syscall", in("rax") number, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rax") ret);
    ret
}
