pub mod devices;
pub mod packet;
pub mod capture;
pub mod pcap;
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

//cat /sys/class/net/wlp7s0/type
//https://www.tcpdump.org/linktypes.html
//65534 - TUN

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use crate::capture::Capture;
    use crate::devices::Device;
    use crate::pcap::pcap::Pcap;

    #[test]
    fn pcap() {

        let pcap = Pcap::from_file("/home/brad/Downloads/EthernetIP-CIP.pcap").expect("Failed to open pcap file");

        /*
        for packet in pcap {
            println!("{:?}", packet);
        }
        */

        println!("{:?}", pcap.get_network());
    }

    #[test]
    fn capture() {
        //NOT TESTABLE WITHOUT ROOT...
        /*
        let devices = Device::list().unwrap();
        println!("{:?}", devices);

        let device = devices.into_iter().find(|d| d.get_name().contains("Ethernet adapter Ethernet:")).unwrap();
        println!("{:?}", device);

        let mut cap = Capture::from_device(&device).unwrap();
        cap.open().unwrap();


        while let Ok(packet) = cap.next_packet() {
            println!("{:x?}", packet);
        }
        */
    }
}
