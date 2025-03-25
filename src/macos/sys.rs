use std::arch::asm;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::os::fd::RawFd;

//pub const SYS_SOCKET: i64 = 0x2000061; // 97
//pub const SYS_CLOSE: i64 = 0x2000006;  // 6
pub const SYS_IOCTL: i64 = 54;  // 98
//pub const SYS_BIND: i64 = 0x2000068;   // 104
//pub const SYS_SENDTO: i64 = 0x200006e; // 110
pub const SYS_READ: i64 = 0x2000003;


//pub const SYS_FCNTL: i64 = 0x200005c; // Correct value for `fcntl` on macOS



pub const IFNAMSIZ: usize = 16;

//pub const SIOCGIFCONF: u64 = 0xc0086914;




pub const AF_INET: i64 = 2;
pub const AF_INET6: i64 = 30;
pub const SOCK_DGRAM: i64 = 2;


pub const CTL_NET: i32 = 4;
pub const AF_ROUTE: i32 = 17;
pub const NET_RT_IFLIST2: i32 = 6; // 3 ???


pub const RTM_NEWADDR: u8 = 0xc;
pub const RTM_IFINFO2: u8 = 0x12;
pub const RTM_NEWMADDR2: u8 = 0x13;


pub const AF_LINK: i32 = 18;

pub const SYS_SYSCTL: usize = 202;

const SYS_SELECT: i64 = 93;

pub const BIOCSETIF: i64 = 0x8020426c;
pub const BIOCIMMEDIATE: i64 = 0x80044270;
pub const BIOCGBLEN: i64 = 0x40044266;
pub const DEFAULT_BPF_BUFFER_SIZE: usize = 4096;









#[repr(C)]
#[derive(Debug)]
pub struct Ifreq {
    pub ifr_name: [u8; 16],
    pub ifr_ifindex: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct SockAddr {
    pub sa_len: u8,
    pub sa_family: u8,
    pub sa_data: [u8; 14],
}

#[repr(C)]
#[derive(Debug)]
pub struct SockAddrInet {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
#[derive(Debug)]
pub struct SockAddrInet6 {
    pub sin6_len: u8,
    pub sin6_family: u8,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u8; 16],
    pub sin6_scope_id: u32,
}






#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RtMsghdr2 {
    rtm_msglen: u16,
    rtm_version: u8,
    rtm_type: u8,
    rtm_index: u16,
    rtm_flags: i32,
    rtm_addrs: i32,
    rtm_refcnt: i32,
    rtm_use: i32,
    rtm_inits: u32,
    rtm_rmx: RtMetrics,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IfMsghdr2 {
    pub ifm_msglen: u16,
    pub ifm_version: u8,
    pub ifm_type: u8,
    pub ifm_addrs: i32,
    pub ifm_flags: u32,
    pub ifm_index: u16,
    pub ifm_snd_len: i32,
    pub ifm_snd_maxlen: i32,
    pub ifm_snd_drops: i32,
    pub ifm_timer: i32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Timeval {
    tv_sec: i64,
    tv_usec: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RtMetrics {
    rmx_locks: u64,
    rmx_mtu: u64,
    rmx_hopcount: u64,
    rmx_expire: u64,
    rmx_recvpipe: u64,
    rmx_sendpipe: u64,
    rmx_ssthresh: u64,
    rmx_rtt: u64,
    rmx_rttvar: u64,
    rmx_pksent: u64,
    rmx_filler: [u64; 3],
}








#[repr(C)]
#[derive(Debug)]
pub struct IfMsghdr {
    pub ifm_msglen: u16, // Message length
    pub ifm_version: u8, // Version (should be 5)
    pub ifm_type: u8,    // Message type (RTM_IFINFO2 = 0x12)
    pub ifm_addrs: u32,  // Bitmap of included address
    pub ifm_flags: u32,  // Interface flags
    pub ifm_index: u16,  // Interface index
    pub ifm_snd_len: u16, // Length of send queue
    pub ifm_snd_maxlen: u16,
    pub ifm_snd_drops: u16,
    pub ifm_timer: u32,
    //_padding: u32,
    //pub ifm_data: IfData64,    // Interface data (e.g., RX/TX bytes, MTU)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IfData64 {
    pub ifi_type: u8,
    pub ifi_typelen: u8,
    pub ifi_physical: u8,
    pub ifi_addrlen: u8,
    pub ifi_hdrlen: u8,
    pub ifi_recvquota: u8,
    pub ifi_xmitquota: u8,
    pub ifi_unused1: u8,
    pub ifi_mtu: u32,
    pub ifi_metric: u32,
    pub ifi_baudrate: u64,
    pub ifi_ipackets: u64,
    pub ifi_ierrors: u64,
    pub ifi_opackets: u64,
    pub ifi_oerrors: u64,
    pub ifi_collisions: u64,
    pub ifi_ibytes: u64,
    pub ifi_obytes: u64,
    pub ifi_imcasts: u64,
    pub ifi_omcasts: u64,
    pub ifi_iqdrops: u64,
    pub ifi_noproto: u64,
    pub ifi_recvtiming: u32,
    pub ifi_xmittiming: u32,
    pub ifi_lastchange_sec: u64,
    pub ifi_lastchange_usec: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct SockAddrDl {
    pub sdl_len: u8,
    pub sdl_family: u8,
    pub sdl_index: u16,
    pub sdl_type: u8,
    pub sdl_nlen: u8, // Name length
    pub sdl_alen: u8,
    pub sdl_slen: u8,
    pub sdl_data: [u8; 12], // Name stored here
}

#[repr(C)]
pub struct TimeVal {
    pub tv_sec: i64,
    pub tv_usec: i64,
}




/*
pub unsafe fn socket(domain: i64, _type: i64, protocol: i64) -> RawFd {
    syscall(SYS_SOCKET, domain, _type, protocol, 0, 0) as RawFd
}

pub unsafe fn bind(fd: RawFd, address: i64, address_len: i64) -> i64 {
    syscall(SYS_BIND, fd as i64, address, address_len, 0, 0)
}
*/
pub unsafe fn ioctl(fd: RawFd, request: i64, arg: i64) -> i64 {
    //syscall(SYS_IOCTL, fd as i64, request, arg, 0, 0)
    let ret: i64;
    asm!(
        "mov x16, {num}",
        "svc #0x80",
        in("x0") fd as u64,
        in("x1") request,
        in("x2") arg,
        num = const SYS_IOCTL,
        lateout("x0") ret,
    );
    ret
}
/*
pub unsafe fn sendto(fd: RawFd, buffer: &mut [u8]) -> i64 {
    syscall(SYS_SENDTO, fd as i64, buffer.as_mut_ptr() as *mut _ as i64, buffer.len() as i64, 0, 0)
}
*/

pub unsafe fn select(nfds: i32, readfds: *mut i32, writefds: *mut i32, exceptfds: *mut i32, timeout: *mut TimeVal) -> i64 {
    let ret: i64;
    asm!(
        "movz x16, #({num} & 0xFFFF)",    // Load lower 16 bits
        "movk x16, #({num} >> 16), lsl #16", // Load upper 16 bits
        "svc #0",                         // Trigger syscall
        in("x0") nfds,                    // nfds in x0
        in("x1") readfds,                 // readfds in x1
        in("x2") writefds,                // writefds in x2
        in("x3") exceptfds,               // exceptfds in x3
        in("x4") timeout,                 // timeout in x4
        lateout("x0") ret,                // Return value in x0
        num = const SYS_SELECT,           // 0x200000C = SYS_select on macOS
    );
    ret
}

pub unsafe fn recvfrom(fd: RawFd, buffer: &mut [u8]) -> isize {
    let ret: isize;
    asm!(
        "movz x16, #({num} & 0xFFFF)",     // Load lower 16 bits
        "movk x16, #({num} >> 16), lsl #16", // Load upper 16 bits
        "svc #0",                         // Trigger syscall
        in("x0") fd as u64,               // FD in x0
        in("x1") buffer.as_mut_ptr(),        // Buffer pointer in x1
        in("x2") buffer.len(),               // Buffer length in x2
        num = const SYS_READ,            // 0x2000003 = SYS_read on macOS
        lateout("x0") ret,                // Return value in x0
    );
    ret
}
/*
pub unsafe fn close(fd: RawFd) {
    syscall(SYS_CLOSE, fd as i64, 0, 0, 0, 0);
}
*/
//for APPLE CHIPS
/*
pub unsafe fn syscall(number: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
    let ret: i64;
    asm!("svc 0", in("x8") number, in("x0") a1, in("x1") a2, in("x2") a3, in("x3") a4, in("x4") a5, lateout("x0") ret);
    ret
}
*/

//#[inline(always)]
pub unsafe fn sysctl(name: &[i32], oldp: *mut u8, oldlenp: *mut usize, newp: *const u8, newlen: usize) -> isize {
    let ret: isize;
    asm!(
        "mov x16, {}",
        "svc #0x80",
        in(reg) SYS_SYSCTL,
        inout("x0") name.as_ptr() as usize => ret,
        in("x1") name.len(),
        in("x2") oldp as usize,
        in("x3") oldlenp as usize,
        in("x4") newp as usize,
        in("x5") newlen,
        options(nostack)
    );
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
