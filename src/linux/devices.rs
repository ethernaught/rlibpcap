use std::{io, mem};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::utils::interface_flags::InterfaceFlags;
use crate::linux::sys::{parse_ip, IfreqAddr, IfreqHwAddr, IfreqIndex, SockAddr, AF_INET, AF_INET6, IFNAMSIZ, SIOCGIFHWADDR, SIOCGIFINDEX, SOCK_DGRAM};
use crate::linux::sys::{close, ioctl, socket, IfConf, IfreqFlags, SIOCGIFCONF, SIOCGIFFLAGS};
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
        let fd = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut ifreqs: [IfreqAddr; 32] = unsafe { mem::zeroed() };
        let mut ifc = IfConf {
            ifc_len: (mem::size_of::<IfreqAddr>() * ifreqs.len()) as i32,
            ifc_buf: ifreqs.as_mut_ptr()
        };


        let res = unsafe { ioctl(fd, SIOCGIFCONF as i64, &mut ifc as *mut _ as i64) };

        if res < 0 {
            unsafe { close(fd) };
            return Err(io::Error::last_os_error());
        }

        let mut devices = Vec::new();
        let count = ifc.ifc_len as usize / mem::size_of::<IfreqAddr>();

        for i in 0..count {
            let ifr = &ifreqs[i];

            let name = String::from_utf8_lossy(&ifr.ifr_name)
                .trim_end_matches('\0')
                .to_string();

            let address = parse_ip(&ifr.ifr_addr);

            let mut ifr_index = IfreqIndex {
                ifr_name: ifr.ifr_name,
                ifr_ifindex: 0
            };

            //CHECK INTERFACE INDEX
            if unsafe { ioctl(fd, SIOCGIFINDEX as i64, &mut ifr_index as *const _ as i64) } < 0 {
                continue;
            }

            let index = ifr_index.ifr_ifindex;

            let mut ifreq_hwaddr = IfreqHwAddr {
                ifr_name: ifr.ifr_name,
                ifr_hwaddr: SockAddr {
                    sa_family: 0,
                    sa_data: [0; 14]
                }
            };

            if unsafe { ioctl(fd, SIOCGIFHWADDR as i64, &mut ifreq_hwaddr as *mut _ as i64) } < 0 {
                return Err(io::Error::last_os_error());
            }

            let data_link_type = Some(unsafe { DataLinkTypes::from_sll2_code(ifreq_hwaddr.ifr_hwaddr.sa_family)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))? }).unwrap();

            let mac_bytes = unsafe { ifreq_hwaddr.ifr_hwaddr.sa_data };
            let mac = Some(EthernetAddress::new(mac_bytes[0], mac_bytes[1], mac_bytes[2], mac_bytes[3], mac_bytes[4], mac_bytes[5]));

            let mut ifr_flags = IfreqFlags {
                ifr_name: ifr.ifr_name,
                ifr_flags: 0
            };

            if unsafe { ioctl(fd, SIOCGIFFLAGS as i64, &mut ifr_flags as *mut _ as i64) } < 0 {
                continue;
            }

            let flags = InterfaceFlags::from_code(ifr_flags.ifr_flags as u32);

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

    pub fn get_mac(&self) -> Option<EthernetAddress> {
        self.mac
    }

    pub fn get_flags(&self) -> Vec<InterfaceFlags> {
        self.flags.clone()
    }
}
