use std::{io, mem};
use std::net::IpAddr;
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
    //flags: Vec<InterfaceFlags>
}

impl Device {

    pub fn list2() -> io::Result<Vec<Self>> {

        let devices = Vec::new();


        Ok(devices)
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

        let res = unsafe { ioctl(fd, 0xc0086914, &mut ifc as *mut _ as i64) };

        if res < 0 {
            //unsafe { close(fd) };
            return Err(io::Error::last_os_error());
        }

        let mut devices = Vec::new();
        let count = ifc.ifc_len as usize / mem::size_of::<IfreqAddr>();

        println!("{}", count);

        /*
        for i in 0..count {
            let ifr = &ifreqs[i];

            let name = String::from_utf8_lossy(&ifr.ifr_name)
                .trim_end_matches('\0')
                .to_string();

            let address = parse_ip(&ifr.ifr_addr);


            let mut ifr_index = IfreqIndex {
                ifr_name: ifr.ifr_name,
                ifr_ifindex: 0,
            };

            //CHECK INTERFACE INDEX
            if unsafe { ioctl(fd, SIOCGIFINDEX as i64, &mut ifr_index as *const _ as i64) } < 0 {
                continue;
            }

            let index = ifr_index.ifr_ifindex;

            let mut ifreq_hwaddr = IfreqHwAddr {
                ifr_name: ifr.ifr_name,
                ifr_hwaddr: SockAddr { sa_family: 0, sa_data: [0; 14] },
            };

            if unsafe { ioctl(fd, SIOCGIFHWADDR as i64, &mut ifreq_hwaddr as *mut _ as i64) } < 0 {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to get hardware address"));
            }

            let data_link_type = Some(unsafe { DataLinkTypes::from_code(ifreq_hwaddr.ifr_hwaddr.sa_family as u32)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))? }).unwrap();

            let mac_bytes = unsafe { ifreq_hwaddr.ifr_hwaddr.sa_data };
            let mac = EthernetAddress::new(mac_bytes[0], mac_bytes[1], mac_bytes[2], mac_bytes[3], mac_bytes[4], mac_bytes[5]);

            /*
            let mut ifr_flags = IfreqFlags {
                ifr_name: ifr.ifr_name,
                ifr_flags: 0
            };

            if unsafe { ioctl(fd, SIOCGIFFLAGS as i64, &mut ifr_flags as *mut _ as i64) } < 0 {
                continue;
            }

            let flags = InterfaceFlags::from_code(ifr_flags.ifr_flags as u16);
            */

            devices.push(Self {
                name,
                address,
                index,
                data_link_type,
                mac,
                //flags
            });
        }
        */

        //unsafe { close(fd) };
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

    /*
    pub fn get_flags(&self) -> Vec<InterfaceFlags> {
        self.flags.clone()
    }
    */
}
