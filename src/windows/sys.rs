pub const AF_UNSPEC: u32 = 0;
pub const GAA_FLAG_SKIP_ANYCAST: u32 = 0x2;
pub const GAA_FLAG_SKIP_MULTICAST: u32 = 0x4;
pub const GAA_FLAG_SKIP_DNS_SERVER: u32 = 0x8;
pub const ERROR_BUFFER_OVERFLOW: u32 = 111;

#[repr(C)]
#[derive(Debug)]
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
