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

/*
#[derive(Debug)]
struct Address {
    addr: IpAddr,
    netmask: Option<IpAddr>,
    broadcast_addr: Option<IpAddr>,
    dst_addr: Option<IpAddr>,
}

#[derive(Debug)]
struct DeviceFlags {
    if_flags: DeviceFlag,
    connection_status: ConnectionStatus,
}

#[derive(Debug)]
enum DeviceFlag {
    UP,
    RUNNING,
    WIRELESS,
    // Add other flags as necessary
}

#[derive(Debug)]
enum ConnectionStatus {
    Connected,
    Disconnected,
}
*/

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

    #[cfg(target_os = "macos")]
    pub fn list() -> io::Result<Vec<Device>> {
        use std::process::Command;

        let output = Command::new("ifconfig")
            .arg("-l")
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to get interfaces"));
        }

        let interfaces = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        for iface in interfaces.split_whitespace() {
            let output = Command::new("ifconfig")
                .arg(iface)
                .output()?;

            if !output.status.success() {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to get interface details"));
            }

            let details = String::from_utf8_lossy(&output.stdout);

            /*
            let addresses = vec![
                Address {
                    addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 129)), // Example IP
                    netmask: Some(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0))),
                    broadcast_addr: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 255))),
                    dst_addr: None,
                },
            ];
            */

            devices.push(Device {
                name: iface.to_string(),
                description: None,
                interface: Interfaces::Ethernet,
                /*
                addresses,
                flags: DeviceFlags {
                    if_flags: DeviceFlag::RUNNING,
                    connection_status: ConnectionStatus::Connected,
                },
                */
            });
        }

        Ok(devices)
    }

    #[cfg(target_os = "windows")]
    pub fn list() -> io::Result<Vec<Device>> {
        use std::process::Command;

        let output = Command::new("ipconfig")
            .output()?;

        if !output.status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to get interfaces"));
        }

        let interfaces = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        for line in interfaces.lines() {
            if line.contains("adapter") {
                let iface_name = line.trim(); // Extract interface name from the output

                /*
                // Simplified address parsing for the example
                let addresses = vec![
                    Address {
                        addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 129)),
                        netmask: Some(IpAddr::V4(Ipv4Addr::new(255, 255, 255, 0))),
                        broadcast_addr: Some(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 255))),
                        dst_addr: None,
                    },
                ];
                */

                devices.push(Device {
                    name: iface_name.to_string(),
                    description: None,
                    interface: Interfaces::Ethernet,
                    /*
                    addresses,
                    flags: DeviceFlags {
                        if_flags: DeviceFlag::RUNNING,
                        connection_status: ConnectionStatus::Connected,
                    },
                    */
                });
            }
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
