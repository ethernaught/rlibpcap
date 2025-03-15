pub mod packet;
pub mod pcap;
#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;













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
    use crate::pcap::pcapng::PcapNg;

    #[test]
    fn pcap() {
        let pcap = Pcap::from_file("/home/brad/Downloads/traffic_capture.pcap").expect("Failed to open pcap file");

        /*
        for packet in pcap {
            println!("{:?}", packet);
        }
        */

        println!("{:?}", pcap);
    }

    #[test]
    fn devices() {
        let devices = Device::list().unwrap();


        println!("{:?}", devices);
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
