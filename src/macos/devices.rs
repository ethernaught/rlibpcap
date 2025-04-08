use std::{io, mem, ptr};
use std::ffi::{c_int, c_void};
use std::net::IpAddr;
use crate::utils::interface_flags::InterfaceFlags;
use crate::macos::sys::{ioctl, sysctl, IfData64, SockAddrDl, AF_LINK, AF_ROUTE, CTL_NET, NET_RT_IFLIST2, RTM_NEWADDR, RTM_IFINFO2, RTM_NEWMADDR2, SOCK_DGRAM, IfMsghdr2};
use crate::utils::data_link_types::DataLinkTypes;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;

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
                            mac: None,
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

    pub fn get_mac(&self) -> Option<EthernetAddress> {
        self.mac
    }

    pub fn get_flags(&self) -> Vec<InterfaceFlags> {
        self.flags.clone()
    }
}
