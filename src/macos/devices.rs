use std::{io, mem};
use std::net::IpAddr;
use crate::interface_flags::InterfaceFlags;
use crate::macos::sys::{ioctl, socket, IfConf, IfreqAddr, AF_INET, SOCK_DGRAM};
use crate::packet::inter::data_link_types::DataLinkTypes;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;

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

    pub fn list() -> io::Result<Vec<Self>> {


        let devices = Vec::new();


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

    pub fn get_mac(&self) -> EthernetAddress {
        self.mac
    }

    pub fn get_flags(&self) -> Vec<InterfaceFlags> {
        self.flags.clone()
    }
}

/*
use std::ffi::{CStr};
use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr;

unsafe extern "C" {
    fn sysctl(
        name: *const c_int,
        namelen: u32,
        oldp: *mut c_void,
        oldlenp: *mut usize,
        newp: *mut c_void,
        newlen: usize,
    ) -> c_int;
}



const CTL_NET: c_int = 4;
const AF_ROUTE: c_int = 17;
const NET_RT_IFLIST2: c_int = 6;

fn main() {
    let mib: [c_int; 6] = [CTL_NET, AF_ROUTE, 0, 0, NET_RT_IFLIST2, 0];
    let mut size: usize = 0;

    // First sysctl to get the size
    unsafe {
        if sysctl(
            mib.as_ptr(),
            mib.len() as u32,
            ptr::null_mut(),
            &mut size,
            ptr::null_mut(),
            0,
        ) != 0
        {
            eprintln!("sysctl failed to get size");
            return;
        }
    }

    println!("sysctl returned {} entries", size);




    // Allocate memory for the interface list
    let mut buffer: Vec<u8> = vec![0; size];

    // Second sysctl to get the actual data
    unsafe {
        if sysctl(
            mib.as_ptr(),
            mib.len() as u32,
            buffer.as_mut_ptr() as *mut c_void,
            &mut size,
            ptr::null_mut(),
            0,
        ) != 0
        {
            eprintln!("sysctl failed to get interface data");
            return;
        }
    }

    println!("{:x?}", buffer);





}
*/
