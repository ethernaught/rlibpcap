use std::{io, mem};
use std::net::IpAddr;
use crate::linux::sys::{Ifreq, IfreqAddr, AF_INET, SIOCGIFHWADDR, SIOCGIFINDEX, SOCK_DGRAM};
use crate::linux::sys::{close, ioctl, parse_ip, socket, IfConf, IfreqFlags, SIOCGIFCONF, SIOCGIFFLAGS};
use crate::packet::inter::data_link_types::DataLinkTypes;
use crate::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;

#[derive(Clone, Debug)]
pub struct Device {
    name: String,
    address: Option<IpAddr>,
    index: i32,
    data_link_type: DataLinkTypes,
    mac: EthernetAddress,
    flags: u16
}

impl Device {

    pub fn list() -> io::Result<Vec<Self>> {
        let fd = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };
        if fd < 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to create socket"));
        }

        let mut ifreqs: [IfreqAddr; 32] = unsafe { mem::zeroed() };
        let mut ifc = IfConf {
            ifc_len: (mem::size_of::<IfreqAddr>() * ifreqs.len()) as i32,
            ifc_buf: ifreqs.as_mut_ptr()
        };


        let res = unsafe { ioctl(fd, SIOCGIFCONF as i64, &mut ifc as *mut _ as i64) };

        if res < 0 {
            unsafe { close(fd) };
            return Err(io::Error::new(io::ErrorKind::InvalidData, "ioctl SIOCGIFCONF failed"));
        }

        let mut devices = Vec::new();
        let count = ifc.ifc_len as usize / mem::size_of::<IfreqAddr>();

        for i in 0..count {
            let ifr = &ifreqs[i];

            let name = String::from_utf8_lossy(&ifr.ifr_name)
                .trim_end_matches('\0')
                .to_string();

            let address = parse_ip(&ifr.ifr_addr);


            let mut ifreq: Ifreq = unsafe { mem::zeroed() };

            let if_name_bytes = name.as_bytes();
            ifreq.ifr_name[..if_name_bytes.len()].copy_from_slice(&if_name_bytes);


            //CHECK INTERFACE INDEX
            if unsafe { ioctl(fd, SIOCGIFINDEX as i64, &mut ifreq as *const _ as i64) } < 0 {
                continue;
            }

            let index = unsafe { ifreq.ifr_ifru.ifru_ifindex };

            //CHECK DATA LINK TYPE
            if unsafe { ioctl(fd, SIOCGIFHWADDR as i64, &mut ifreq as *mut _ as i64) } < 0 {
                continue;
            }

            let data_link_type = Some(unsafe { DataLinkTypes::from_code(ifreq.ifr_ifru.ifru_hwaddr.sa_family as u32)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))? }).unwrap();

            let ether_bytes = unsafe { ifreq.ifr_ifru.ifru_hwaddr.sa_data };
            let mac = EthernetAddress::new(ether_bytes[0], ether_bytes[1], ether_bytes[2], ether_bytes[3], ether_bytes[4], ether_bytes[5]);

            //CHECK FLAGS
            let mut ifr_flags = IfreqFlags {
                ifr_name: ifr.ifr_name,
                ifr_flags: 0
            };

            if unsafe { ioctl(fd, SIOCGIFFLAGS as i64, &mut ifr_flags as *mut _ as i64) } < 0 {
                continue;
            }

            let flags = ifr_flags.ifr_flags as u16;

            devices.push(Self {
                name,
                address,
                index,
                data_link_type,
                mac,
                flags
            });
        }

        unsafe { close(fd) };
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

    pub fn get_flags(&self) -> u16 {
        self.flags
    }
}
