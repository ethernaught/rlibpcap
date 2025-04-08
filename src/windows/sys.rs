
use std::ptr;
use std::mem;
use std::net::Ipv4Addr;
use std::ffi::c_void;
use std::os::raw::{c_char, c_int};
use std::io::Error;

pub const AF_UNSPEC: u32 = 0;
pub const GAA_FLAG_SKIP_ANYCAST: u32 = 0x2;
pub const GAA_FLAG_SKIP_MULTICAST: u32 = 0x4;
pub const GAA_FLAG_SKIP_DNS_SERVER: u32 = 0x8;
pub const ERROR_BUFFER_OVERFLOW: u32 = 111;
pub const AF_INET: c_int = 2;
pub const SOCK_RAW: c_int = 3;
pub const IPPROTO_IP: c_int = 0;
pub const SIO_RCVALL: u32 = 0x98000001;
pub const RCVALL_ON: u32 = 1;


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IpAdapterAddressedLh {
    pub length: u32,
    pub if_index: u32,
    pub next: *mut Self,
    pub adapter_name: *mut u8,
    pub first_unicast_address: *mut u32,
    pub first_anycast_address: *mut u32,
    pub first_multicast_address: *mut u32,
    pub first_dns_server_address: *mut u32,
    pub dns_suffix: *mut u16,
    pub description: *mut u16,
    pub friendly_name: *mut u16,
    pub physical_address: [u8; 8],
    pub physical_address_length: u32,
    pub flags: u32,
    pub mtu: u32,
    pub if_type: u32,
    pub oper_status: u32,
    pub ipv6_if_index: u32,
    pub zone_indices: [u32; 16],
    pub first_prefix: *mut u32
}

#[link(name = "iphlpapi")]
extern "system" {

    pub fn GetAdaptersAddresses(family: u32, flags: u32, reserved: *mut u32, adapter_addresses: *mut IpAdapterAddressedLh, size_pointer: *mut u32) -> u32;
}












pub type SOCKET = usize;
pub type BYTE = u8;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WsaData {
    pub wVersion: u16,
    pub wHighVersion: u16,
    pub szDescription: [c_char; 257],
    pub szSystemStatus: [c_char; 129],
    pub iMaxSockets: u16,
    pub iMaxUdpDg: u16,
    pub lpVendorInfo: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

extern "system" {
    pub fn WSAStartup(wVersionRequested: u16, lpWSAData: *mut WsaData) -> c_int;
    pub fn socket(af: c_int, kind: c_int, protocol: c_int) -> SOCKET;
    pub fn bind(s: SOCKET, name: *const SockAddr, namelen: c_int) -> c_int;
    pub fn WSAIoctl(
s: SOCKET,
dwIoControlCode: u32,
lpvInBuffer: *mut c_void,
cbInBuffer: u32,
lpvOutBuffer: *mut c_void,
cbOutBuffer: u32,
lpcbBytesReturned: *mut u32,
lpOverlapped: *mut c_void,
lpCompletionRoutine: Option<extern "system" fn()>
    ) -> c_int;
    pub fn recvfrom(s: SOCKET, buf: *mut c_char, len: c_int, flags: c_int, from: *mut SockAddr, fromlen: *mut c_int) -> c_int;
}








