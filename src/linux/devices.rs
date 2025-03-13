use std::io;
use crate::packet::inter::data_link_types::DataLinkTypes;

/*
[
    Device {
        name: "wlp7s0",
        desc: None,
        addresses: [
            Address {
                addr: 192.168.0.129,
                netmask: Some(255.255.255.0),
                broadcast_addr: Some(192.168.0.255),
                dst_addr: None
            },
            Address {
                addr: xxx:xxx:xxx:xxx,
                netmask: Some(ffff:ffff:ffff:ffff::),
                broadcast_addr: None,
                dst_addr: None
            }
        ],
        flags: DeviceFlags {
            if_flags: UP | RUNNING | WIRELESS, connection_status: Connected
        },
        type: DataLinkType
    }
]
*/

#[derive(Clone, Debug)]
pub struct Device {
    name: String,
    description: Option<String>,
    data_link_type: DataLinkTypes
    //addresses: Vec<Address>,
    //flags: DeviceFlags
}

impl Device {

    pub fn list() -> io::Result<Vec<Device>> {
        let mut devices = Vec::new();






        Ok(devices)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }
}




/*
pub const SYS_SOCKET: i64 = 41;
pub const AF_PACKET: i64 = 17;
pub const SOCK_RAW: i64 = 3;
pub const ETH_P_ALL: u16 = 0x0003;
pub const SOL_SOCKET: i64 = 1;
pub const SO_BINDTODEVICE: i64 = 25;
pub const SYS_IOCTL: i64 = 16;
pub const SYS_BIND: i64 = 49;
pub const SYS_SENDTO: i64 = 0x2C;
pub const SYS_RECV_FROM: i64 = 45;
pub const SYS_SET_SOCK_OPT: i64 = 54;
pub const IFNAMSIZ: usize = 16;
pub const SIOCGIFINDEX: u64 = 0x8933;
pub const SIOCGIFHWADDR: u64 = 0x00008927;

pub const AF_INET: i64 = 2;
pub const SOCK_DGRAM: i64 = 2;

#[repr(C)]
pub struct IfReq {
    ifr_name: [u8; IFNAMSIZ],
    ifr_ifru: IfrIfru
}

#[repr(C)]
pub union IfrIfru {
    pub ifru_addr: SockAddr,
    pub ifru_dstaddr: SockAddr,
    pub ifru_broadaddr: SockAddr,
    pub ifru_netmask: SockAddr,
    pub ifru_hwaddr: SockAddr,
    pub ifru_flags: i16,
    pub ifru_ifindex: i32,
    pub ifru_metric: i32,
    pub ifru_mtu: i32,
    pub ifru_map: u16,
    pub ifru_slave: [i8; 16],
    pub ifru_newname: [i8; 16],
    pub ifru_data: *mut i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SockAddr {
    pub sa_family: u16,
    pub sa_data: [i8; 14],
}

fn get_interface_type(interface: &str) -> io::Result<()> {
    let fd = unsafe { syscall(SYS_SOCKET, AF_INET, SOCK_DGRAM, 0, 0, 0) };
    //let fd = unsafe { syscall(SYS_SOCKET, AF_PACKET as i64, SOCK_RAW as i64, ETH_P_ALL.to_be() as i64, 0, 0) };

    if fd < 0 {
        println!("A");
        return Err(io::Error::last_os_error());
    }

    let mut ifr: IfReq = unsafe { mem::zeroed() };

    let if_name_bytes = interface.to_string().into_bytes();
    if if_name_bytes.len() >= IFNAMSIZ {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
    }

    ifr.ifr_name[..if_name_bytes.len()].copy_from_slice(&if_name_bytes);

    let res = unsafe {
        syscall(SYS_IOCTL, fd as i64, SIOCGIFHWADDR as i64, &mut ifr as *mut _ as i64, 0, 0)
    };

    if res < 0 {
        return Err(io::Error::last_os_error());
    }


    unsafe {

        println!("Hardware address: {:?}", ifr.ifr_ifru.ifru_hwaddr);
        let link_type = ifr.ifr_ifru.ifru_hwaddr.sa_family as i32; // Extract sa_family
        match link_type {
            1 => println!("Interface type: Ethernet"),
            772 => println!("Interface type: Loopback"),
            801 => println!("Interface type: Wi-Fi"),
            _ => println!("Interface type: Unknown ({})", link_type),
        }

    }

    unsafe { syscall(3, fd, 0, 0, 0, 0) }; // SYS_CLOSE

    Ok(())
}
*/
