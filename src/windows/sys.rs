use std::net::{Ipv4Addr, Ipv6Addr};

pub const AF_UNSPEC: u32 = 0;
pub const GAA_FLAG_SKIP_ANYCAST: u32 = 0x2;
pub const GAA_FLAG_SKIP_MULTICAST: u32 = 0x4;
pub const GAA_FLAG_SKIP_DNS_SERVER: u32 = 0x8;
pub const ERROR_BUFFER_OVERFLOW: u32 = 111;
pub const AF_INET: i32 = 2;
pub const SOCK_RAW: i32 = 3;
pub const IPPROTO_IP: i32 = 0;
pub const SIO_RCVALL: u32 = 0x98000001;
pub const RCVALL_ON: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddrLh {
    pub length: u32,
    pub next: *mut SockAddrLh,
    pub unsure: [u8; 52],
    pub address: Ipv4Addr
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddrIn {
    pub sa_family: u32,
    pub sin_addr: [u8; 4],
    pub sin_zero: [u8; 8]
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14]
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IpAdapterAddressedLh {
    pub length: u32,
    pub if_index: u32,
    pub next: *mut Self,
    pub adapter_name: *mut u8,
    pub first_unicast_address: *mut SockAddrLh,
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WsaData {
    pub w_version: u16,
    pub w_high_version: u16,
    pub sz_description: [i8; 257],
    pub sz_system_status: [i8; 129],
    pub i_max_sockets: u16,
    pub i_max_udp_dg: u16,
    pub lp_vendor_info: *mut i8
}

#[link(name = "iphlpapi")]
extern "system" {
    pub fn GetAdaptersAddresses(family: u32, flags: u32, reserved: *mut u32, adapter_addresses: *mut IpAdapterAddressedLh, size_pointer: *mut u32) -> u32;
    pub fn WSAStartup(wVersionRequested: u16, lpWSAData: *mut WsaData) -> i32;
    pub fn socket(af: i32, kind: i32, protocol: i32) -> usize;
    pub fn bind(s: usize, name: *const SockAddr, namelen: i32) -> i32;
    pub fn WSAIoctl(fd: usize, dw_io_control_code: u32, lpv_in_buffer: *mut u16, cb_in_buffer: u32, lpv_out_buffer: *mut u16, cb_out_buffer: u32, lpcb_bytes_returned: *mut u32, lp_overlapped: *mut u16, lp_completion_routine: Option<extern "system" fn()>) -> i32;
    pub fn recvfrom(s: usize, buf: *mut i8, len: i32, flags: i32, from: *mut SockAddrLl, fromlen: *mut i32) -> i32;
}
