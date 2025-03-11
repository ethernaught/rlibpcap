use std::{fs, io};
use crate::packet::inter::interfaces::Interfaces;

#[derive(Clone, Debug)]
pub struct Device {
    name: String,
    description: Option<String>,
    interface: Interfaces
    //addresses: Vec<Address>,
    //flags: DeviceFlags
}

impl Device {

    #[cfg(target_os = "linux")]
    pub fn list() -> io::Result<Vec<Device>> {
        let mut devices = Vec::new();

        for entry in fs::read_dir("/sys/class/net/")? {
            let entry = entry?;
            let interface_name = entry.file_name().into_string().unwrap();

            let mac_address = fs::read_to_string(format!("/sys/class/net/{}/address", interface_name))?;

            /*
            let mut addresses = Vec::new();
            if let Ok(ipv4_path) = fs::read_to_string(format!("/sys/class/net/{}/address", interface_name)) {
                addresses.push(Address {
                    addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 129)), // Example IP
                    netmask: Some(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0))),
                    broadcast_addr: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 255))),
                    dst_addr: None,
                });
            }

            let flags = DeviceFlags {
                if_flags: DeviceFlag::RUNNING,
                connection_status: ConnectionStatus::Connected,
            };
            */

            devices.push(Self {
                name: interface_name,
                description: Some(mac_address),
                interface: Interfaces::Ethernet,
                //addresses,
                //flags,
            });
        }

        Ok(devices)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn get_interface(&self) -> Interfaces {
        self.interface
    }
}
