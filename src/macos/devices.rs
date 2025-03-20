use std::io;
use std::net::IpAddr;
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

    /*
    pub fn get_flags(&self) -> Vec<InterfaceFlags> {
        self.flags.clone()
    }
    */
}
