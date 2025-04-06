use std::ffi::{CStr};
use std::os::raw::{c_char, c_ulong};
use std::ptr::{null_mut};
use std::{io, slice};
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStringExt;
use std::ffi::OsString;
use std::net::IpAddr;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::utils::data_link_types::DataLinkTypes;
use crate::utils::interface_flags::InterfaceFlags;
use crate::windows::sys::{GetAdaptersAddresses, AF_UNSPEC, GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_DNS_SERVER, GAA_FLAG_SKIP_MULTICAST, PIP_ADAPTER_ADDRESSES_LH, ULONG};

#[derive(Clone, Debug)]
pub struct Device {
    name: String,
    address: Option<IpAddr>,
    index: i32,
    data_link_type: DataLinkTypes,
    mac: EthernetAddress,
    flags: Vec<InterfaceFlags>
}

impl Device {

    pub fn new(name: String, address: Option<IpAddr>, index: i32, data_link_type: DataLinkTypes, mac: EthernetAddress, flags: Vec<InterfaceFlags>) -> Self {
        Self {
            name,
            address,
            index,
            data_link_type,
            mac,
            flags
        }
    }

    pub fn list() -> io::Result<Vec<Self>> {
        unsafe {
            let mut buffer_len: ULONG = 15000;
            let mut buffer: Vec<u8> = vec![0u8; buffer_len as usize];

            let result = GetAdaptersAddresses(
                AF_UNSPEC,
                GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER,
                null_mut(),
                buffer.as_mut_ptr() as PIP_ADAPTER_ADDRESSES_LH,
                &mut buffer_len,
            );

            if result != 0 {
                return Err(io::Error::last_os_error());
            }

            let mut adapter = buffer.as_mut_ptr() as PIP_ADAPTER_ADDRESSES_LH;

            while !adapter.is_null() {
                let uuid = CStr::from_ptr((*adapter).AdapterName).to_string_lossy();

                let fname_ptr = (*adapter).FriendlyName;
                let mut len = 0;
                while *fname_ptr.add(len) != 0 {
                    len += 1;
                }
                let fname_slice = slice::from_raw_parts(fname_ptr, len);
                //let friendly_name = OsString::from_wide(fname_slice).to_string_lossy();

                println!("{} → {}   {:?}", uuid, OsString::from_wide(fname_slice).to_string_lossy(), (*adapter));

                adapter = (*adapter).Next;
            }

            todo!()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_address(&self) -> Option<IpAddr> {
        self.address
    }

    pub fn get_index(&self) -> i32 {
        self.index
    }

    pub fn get_data_link_type(&self) -> DataLinkTypes {
        self.data_link_type
    }

    pub fn get_mac(&self) -> EthernetAddress {
        self.mac
    }

    pub fn get_flags(&self) -> Vec<InterfaceFlags> {
        self.flags.clone()
    }
}
