use std::ffi::{CStr};
use std::os::raw::{c_char, c_ulong};
use std::ptr::{null_mut};
use std::slice;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStringExt;
use std::ffi::OsString;

pub const AF_UNSPEC: u32 = 0;
pub const GAA_FLAG_SKIP_ANYCAST: u32 = 0x2;
pub const GAA_FLAG_SKIP_MULTICAST: u32 = 0x4;
pub const GAA_FLAG_SKIP_DNS_SERVER: u32 = 0x8;
pub const ERROR_BUFFER_OVERFLOW: u32 = 111;

pub type ULONG = c_ulong;
pub type DWORD = u32;
pub type PVOID = *mut std::ffi::c_void;
pub type UINT = u32;
pub type IF_INDEX = u32;
pub type NET_IF_COMPARTMENT_ID = u32;
pub type NET_IF_NETWORK_GUID = [u8; 16];
pub type TUNNEL_TYPE = u32;
pub type ADDRESS_FAMILY = u16;
pub type IF_LUID = u64;
pub type IF_TYPE = u32;
pub type PIP_ADAPTER_ADDRESSES_LH = *mut IP_ADAPTER_ADDRESSES_LH;


#[repr(C)]
#[derive(Debug)]
pub struct IP_ADAPTER_ADDRESSES_LH {
    pub Length: ULONG,
    pub IfIndex: IF_INDEX,
    pub Next: PIP_ADAPTER_ADDRESSES_LH,
    pub AdapterName: *mut c_char,
    pub FirstUnicastAddress: *mut std::ffi::c_void,
    pub FirstAnycastAddress: *mut std::ffi::c_void,
    pub FirstMulticastAddress: *mut std::ffi::c_void,
    pub FirstDnsServerAddress: *mut std::ffi::c_void,
    pub DnsSuffix: *mut u16,
    pub Description: *mut u16,
    pub FriendlyName: *mut u16,
    pub PhysicalAddress: [u8; 8],
    pub PhysicalAddressLength: DWORD,
    pub Flags: DWORD,
    pub Mtu: DWORD,
    pub IfType: IF_TYPE,
    pub OperStatus: DWORD,
    pub Ipv6IfIndex: IF_INDEX,
    pub ZoneIndices: [DWORD; 16],
    pub FirstPrefix: *mut std::ffi::c_void
}

#[link(name = "iphlpapi")]
extern "system" {
    pub fn GetAdaptersAddresses(
        Family: ULONG,
        Flags: ULONG,
        Reserved: PVOID,
        AdapterAddresses: PIP_ADAPTER_ADDRESSES_LH,
        SizePointer: *mut ULONG,
    ) -> ULONG;
}
