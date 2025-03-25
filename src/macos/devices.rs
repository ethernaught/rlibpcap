use std::{io, mem, ptr};
use std::ffi::{c_int, c_void};
use std::net::IpAddr;
use crate::utils::interface_flags::InterfaceFlags;
use crate::macos::sys::{ioctl, sysctl, IfData64, SockAddrDl, AF_LINK, AF_ROUTE, CTL_NET, NET_RT_IFLIST2, RTM_NEWADDR, RTM_IFINFO2, RTM_NEWMADDR2, SOCK_DGRAM, IfMsghdr2};
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
        let mib: [c_int; 6] = [CTL_NET, AF_ROUTE, 0, 0, NET_RT_IFLIST2, 0];
        let mut size: usize = 0;

        let res = unsafe { sysctl(&mib, ptr::null_mut(), &mut size, ptr::null_mut(), 0) };

        if res != 0 {
            return Err(io::Error::last_os_error());
        }

        let mut buf: Vec<u8> = vec![0u8; size];

        let res = unsafe { sysctl(&mib, buf.as_mut_ptr(), &mut size, ptr::null_mut(), 0) };
        if res != 0 {
            return Err(io::Error::last_os_error());
        }

        let mut devices = Vec::new();

        let mut offset = 0;
        while offset < size {
            //let hdr: &IfMsghdr2 = unsafe { &*(buf.as_ptr().add(offset) as *const IfMsghdr2) };
            let hdr = IfMsghdr2 {
                ifm_msglen: u16::from_ne_bytes([buf[offset], buf[offset + 1]]),
                ifm_version: buf[offset + 2],
                ifm_type: buf[offset + 3],
                ifm_addrs: u32::from_ne_bytes([buf[offset + 4], buf[offset + 5], buf[offset + 6], buf[offset + 7]]),
                ifm_flags: u32::from_ne_bytes([buf[offset + 8], buf[offset + 9], buf[offset + 10], buf[offset + 11]]),
                ifm_index: u16::from_ne_bytes([buf[offset + 12], buf[offset + 13]]),
                ifm_snd_len: u32::from_ne_bytes([buf[offset + 14], buf[offset + 15], buf[offset + 16], buf[offset + 17]]),
                ifm_snd_maxlen: u32::from_ne_bytes([buf[offset + 18], buf[offset + 19], buf[offset + 20], buf[offset + 21]]),
                ifm_snd_drops: u32::from_ne_bytes([buf[offset + 22], buf[offset + 23], buf[offset + 24], buf[offset + 25]]),
                ifm_timer: u32::from_ne_bytes([buf[offset + 26], buf[offset + 27], buf[offset + 28], buf[offset + 29]]),
            };

            println!("{:?}", hdr);

            match hdr.ifm_type {
                RTM_NEWADDR => {
                    //println!("NEW_ADDR {:x?}", &buffer[offset+28..offset+hdr.ifm_msglen as usize-28]);

                }
                RTM_IFINFO2 => {
                    let data = IfData64 {
                        ifi_type: buf[offset + 32],
                        ifi_typelen: buf[offset + 33],
                        ifi_physical: buf[offset + 34],
                        ifi_addrlen: buf[offset + 35],
                        ifi_hdrlen: buf[offset + 36],
                        ifi_recvquota: buf[offset + 37],
                        ifi_xmitquota: buf[offset + 38],
                        ifi_unused1: buf[offset + 39],
                        ifi_mtu: u32::from_ne_bytes(buf[offset + 40..offset + 44].try_into().unwrap()),
                        ifi_metric: u32::from_ne_bytes(buf[offset + 44..offset + 48].try_into().unwrap()),
                        ifi_baudrate: u64::from_ne_bytes(buf[offset + 48..offset + 56].try_into().unwrap()),
                        ifi_ipackets: u64::from_ne_bytes(buf[offset + 56..offset + 64].try_into().unwrap()),
                        ifi_ierrors: u64::from_ne_bytes(buf[offset + 64..offset + 72].try_into().unwrap()),
                        ifi_opackets: u64::from_ne_bytes(buf[offset + 72..offset + 80].try_into().unwrap()),
                        ifi_oerrors: u64::from_ne_bytes(buf[offset + 80..offset + 88].try_into().unwrap()),
                        ifi_collisions: u64::from_ne_bytes(buf[offset + 88..offset + 96].try_into().unwrap()),
                        ifi_ibytes: u64::from_ne_bytes(buf[offset + 96..offset + 104].try_into().unwrap()),
                        ifi_obytes: u64::from_ne_bytes(buf[offset + 104..offset + 112].try_into().unwrap()),
                        ifi_imcasts: u64::from_ne_bytes(buf[offset + 112..offset + 120].try_into().unwrap()),
                        ifi_omcasts: u64::from_ne_bytes(buf[offset + 120..offset + 128].try_into().unwrap()),
                        ifi_iqdrops: u64::from_ne_bytes(buf[offset + 128..offset + 136].try_into().unwrap()),
                        ifi_noproto: u64::from_ne_bytes(buf[offset + 136..offset + 144].try_into().unwrap()),
                        ifi_recvtiming: u32::from_ne_bytes(buf[offset + 144..offset + 148].try_into().unwrap()),
                        ifi_xmittiming: u32::from_ne_bytes(buf[offset + 148..offset + 152].try_into().unwrap()),
                        ifi_lastchange_sec: u64::from_ne_bytes(buf[offset + 152..offset + 160].try_into().unwrap()),
                        ifi_lastchange_usec: u64::from_ne_bytes(buf[offset + 160..offset + 168].try_into().unwrap()),
                    };

                    let sdl = SockAddrDl {
                        sdl_len: buf[offset+hdr.ifm_msglen as usize - 20],
                        sdl_family: buf[offset+hdr.ifm_msglen as usize - 19],
                        sdl_index: u16::from_ne_bytes([buf[offset+hdr.ifm_msglen as usize - 18], buf[offset+hdr.ifm_msglen as usize - 17]]),
                        sdl_type: buf[offset+hdr.ifm_msglen as usize - 16],
                        sdl_nlen: buf[offset+hdr.ifm_msglen as usize - 15],
                        sdl_alen: buf[offset+hdr.ifm_msglen as usize - 14],
                        sdl_slen: buf[offset+hdr.ifm_msglen as usize - 13],
                        sdl_data: buf[offset+hdr.ifm_msglen as usize - 12..offset+hdr.ifm_msglen as usize].try_into().unwrap()
                    };

                    if sdl.sdl_family == AF_LINK as u8 {
                        let name_len = sdl.sdl_nlen as usize;
                        let name_bytes = &sdl.sdl_data[0..name_len];
                        let name = String::from_utf8_lossy(name_bytes).to_string();

                        let data_link_type = DataLinkTypes::from_sdl_code(data.ifi_type)
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

                        devices.push(Self {
                            name,
                            address: None,
                            index: hdr.ifm_index as i32,
                            data_link_type,
                            mac: EthernetAddress::new(0, 0, 0, 0, 0, 0),
                            flags: InterfaceFlags::from_code(hdr.ifm_flags)
                        });
                    }
                }
                RTM_NEWMADDR2 => {
                    //println!("NEW_MADDR   {:x?}", &buffer[offset+28..offset+hdr.ifm_msglen as usize-28]);

                    /*
                    let sdl: &SockAddr = unsafe {
                        &*(buffer.as_ptr().add(offset+28) as *const SockAddr)
                    };

                    println!("{:?}", sdl);
                    */

                }
                _ => {}
            }

            offset += hdr.ifm_msglen as usize;
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

    let mut offset = 0;

    while offset < buffer.len() {
        if buffer.len() < offset + mem::size_of::<IfMsghdr>() {
            break;
        }

        let ifm: &IfMsghdr = unsafe {
            &*(buffer.as_ptr().add(offset) as *const IfMsghdr)
        };

        println!("Interface Message: {:?}", ifm);

        offset += ifm.ifm_msglen as usize;
    }





}



#[repr(C)]
#[derive(Debug)]
struct IfMsghdr {
    ifm_msglen: u16, // Length of the message
    ifm_version: u8, // Message version
    ifm_type: u8,    // Message type (RTM_IFINFO, RTM_NEWADDR, etc.)
    ifm_addrs: i32,  // Bitmask of addresses
    ifm_flags: i32,  // Interface flags
    ifm_index: u16,  // Interface index
    ifm_snd_len: u32,
    ifm_snd_maxlen: u32,
    ifm_snd_drops: u32,
    ifm_timer: u32,
    ifm_data: IfData,
}

#[repr(C)]
#[derive(Debug)]
struct IfData {
    ifi_type: u8,   // Interface type
    ifi_typelen: u8,
    ifi_physical: u8,
    ifi_addrlen: u8,
    ifi_hdrlen: u8,
    ifi_recvquota: u8,
    ifi_xmitquota: u8,
    ifi_unused1: u8,
    ifi_mtu: u32,
    ifi_metric: u32,
    ifi_baudrate: u64,
    ifi_ipackets: u32,
    ifi_ierrors: u32,
    ifi_opackets: u32,
    ifi_oerrors: u32,
    ifi_collisions: u32,
    ifi_ibytes: u32,
    ifi_obytes: u32,
    ifi_imcasts: u32,
    ifi_omcasts: u32,
    ifi_iqdrops: u32,
    ifi_noproto: u32,
    ifi_recvtiming: u32,
    ifi_xmittiming: u32,
    ifi_lastchange_sec: i32,
    ifi_lastchange_usec: i32,
}
*/




//METHOD 2





/*
use std::ffi::{CStr};
use std::mem;
use std::net::Ipv4Addr;
use std::os::raw::{c_int, c_void};
use std::ptr;
use libc::{AF_INET};

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


    unsafe {
        parse_interfaces(&buffer);
    }

}

//const RTM_IFINFO: u8 = 0x12;
const RTM_IFINFO: u8 = 18;  // Interface information message
const RTM_NEWADDR: u8 = 12; // New address assigned
const RTM_DELADDR: u8 = 19; // Address removed


unsafe fn parse_interfaces(buffer: &[u8]) {
    let mut offset = 0;
    while offset < buffer.len() {
        let hdr_ptr = buffer.as_ptr().add(offset) as *const IfMsghdr;
        let hdr = &*hdr_ptr;


        match hdr.ifm_type {
            RTM_IFINFO => {
                println!("Interface Information message");
                handle_rtm_ifinfo(hdr);
                // Handle the interface info here
            },
            _ => {
                //println!("Unknown message type: {}", hdr.ifm_type);
            },
        }



        // Check for message type: RTM_IFINFO
        //if hdr.ifm_type == RTM_IFINFO {
            //println!("Interface Index: {}", hdr.ifm_index);
            //println!("MTU: {}", hdr.ifm_data.ifi_mtu);
            //println!("RX Bytes: {}", hdr.ifm_data.ifi_ibytes);
            //println!("TX Bytes: {}", hdr.ifm_data.ifi_obytes);

            // Parse the interface name (first sockaddr in the message)

            //println!("{:?}", hdr);

            // Interface name (usually found in the first sockaddr structure)
            //if let Some(ifname) = get_ifname(sockaddr_ptr) {
            //    println!("Interface Name: {}", ifname);
            //}
        //}

        // Move to the next message based on ifm_msglen
        offset += hdr.ifm_msglen as usize;
    }
}





// Define the IfMsghdr structure (simplified)
#[repr(C)]
#[derive(Debug)]
struct IfMsghdr {
    ifm_msglen: u16,    // Length of the message
    ifm_version: u8,     // Version number
    ifm_type: u8,        // Message type (RTM_IFINFO, etc.)
    ifm_addrs: i32,      // Bitmask of addresses (e.g., IFA_ADDRESS)
    ifm_flags: i32,      // Interface flags
    ifm_index: u16,      // Interface index
    ifm_snd_len: u32,
    ifm_snd_maxlen: u32,
    ifm_snd_drops: u32,
    ifm_timer: u32,
    ifm_data: IfData,    // Interface data (e.g., RX/TX bytes, MTU)
}

#[repr(C)]
#[derive(Debug)]
struct IfData {
    ifd_mtu: u32,       // MTU of the interface
    ifd_xmit_speed: u32,
    ifd_recv_speed: u32,
    ifd_ipackets: u32,  // Received packets
    ifd_opackets: u32,  // Transmitted packets
    ifd_ierrors: u32,   // Input errors
    ifd_oerrors: u32,   // Output errors
    ifd_collisions: u32,
    ifd_ibytes: u32,    // Received bytes
    ifd_obytes: u32,    // Transmitted bytes
    ifd_imcasts: u32,
    ifd_opcasts: u32,
    ifd_iqdrops: u32,
    ifd_noproto: u32,
}

unsafe fn handle_rtm_ifinfo(msg: *const IfMsghdr) {
    // First, check the message type to ensure it's RTM_IFINFO
    if (*msg).ifm_type == 18 { // RTM_IFINFO message type
        // Get the interface index from the message
        let if_index = (*msg).ifm_index;
        let if_name = get_if_name(if_index);

        println!("Interface Index: {}", if_index);
        println!("Interface Name: {}", if_name);

        // You can also access other data like MTU, RX/TX bytes:
        let if_data = &(*msg).ifm_data;
        println!("MTU: {}", if_data.ifd_mtu);
        println!("Received Bytes: {}", if_data.ifd_ibytes);
        println!("Transmitted Bytes: {}", if_data.ifd_obytes);
    }
}

// Assuming you have a way to obtain the interface name from the interface index.
// On macOS, you can use sysctl to get the name of an interface by index.
unsafe fn get_if_name(if_index: u16) -> String {

    let mib: [c_int; 6] = [CTL_NET, AF_ROUTE, 0, 0, NET_RT_IFLIST2, if_index as c_int];
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
            return String::new();
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
            return String::new();
        }
    }

    println!("{}", String::from_utf8_lossy(&buffer));

    String::new()
}
*/
