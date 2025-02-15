pub mod capture;
pub mod devices;
pub mod packet;

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
        }
    }
]
*/

#[cfg(test)]
mod tests {
    use crate::capture::Capture;
    use crate::devices::Device;

    #[test]
    fn it_works() {
        let devices = Device::list().unwrap();

        let device = devices.into_iter().find(|d| d.get_name().contains("wlp7s0")).unwrap();
        println!("{:?}", device);

        let mut cap = Capture::from_device(device).unwrap();
        cap.open().unwrap();


        while let Ok(packet) = cap.next_packet() {
            println!("{:x?}", packet);
        }
    }
}
