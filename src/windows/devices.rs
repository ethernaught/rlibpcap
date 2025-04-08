use std::ptr::{null_mut};
use std::{io, slice};
use std::net::IpAddr;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::utils::data_link_types::DataLinkTypes;
use crate::utils::interface_flags::InterfaceFlags;
use crate::windows::sys::{GetAdaptersAddresses, IpAdapterAddressedLh, AF_UNSPEC, GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_DNS_SERVER, GAA_FLAG_SKIP_MULTICAST};

#[derive(Clone, Debug)]
pub struct Device {
    name: String,
    address: Option<IpAddr>,
    index: i32,
    data_link_type: DataLinkTypes,
    mac: Option<EthernetAddress>,
    flags: Vec<InterfaceFlags>
}

impl Device {

    pub fn new(name: String, address: Option<IpAddr>, index: i32, data_link_type: DataLinkTypes, mac: Option<EthernetAddress>, flags: Vec<InterfaceFlags>) -> Self {
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
        let mut buffer_len: u32 = 15000;
        let mut buffer: Vec<u8> = vec![0u8; buffer_len as usize];

        let result = unsafe { GetAdaptersAddresses(AF_UNSPEC, GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER, null_mut(), buffer.as_mut_ptr() as *mut IpAdapterAddressedLh, &mut buffer_len) };

        if result != 0 {
            return Err(io::Error::last_os_error());
        }

        let mut adapter = unsafe { buffer.as_mut_ptr() as *mut IpAdapterAddressedLh };

        let mut devices = Vec::new();

        while !adapter.is_null() {
            let mut ar = unsafe { &mut *adapter };

            let fname = unsafe {
                let fname_ptr = (*adapter).friendly_name;

                let mut len = 0;
                while *fname_ptr.add(len) != 0 {
                    len += 1;
                }

                String::from_utf16_lossy(slice::from_raw_parts(fname_ptr, len))
            };



            let mac = match EthernetAddress::try_from(&ar.physical_address[..ar.physical_address_length as usize]) {
                Ok(mac) => Some(mac),
                Err(_) => None
            };

            devices.push(Self {
                name: fname,
                address: None, //UNICAST ADDRESS...
                index: ar.if_index as i32,
                data_link_type: DataLinkTypes::Null,
                mac,
                flags: InterfaceFlags::from_code(ar.flags)
            });

            adapter = unsafe { (*adapter).next };
        }

        Ok(devices)
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

    pub fn get_mac(&self) -> Option<EthernetAddress> {
        self.mac
    }

    pub fn get_flags(&self) -> Vec<InterfaceFlags> {
        self.flags.clone()
    }
}
