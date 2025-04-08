pub mod packet;
pub mod pcap;
pub mod utils;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;
#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;








//check parse_ip is even required....

//GET MAC ADDRESS & IP ADDRESS - FOR LINUX AND MAC - PER DEVICE
//ANY FOR MAC

/* WINDOWS
Goal	Path	Dependency	Usable in Rust without crates?
IP packets (no MAC)	Raw sockets	None	✅
Ethernet frames (L2)	Npcap driver	Requires Npcap	❌
Ethernet frames (L2)	Windows Filtering Platform	WinAPI + config	❌ (limited L2)
Ethernet frames (L2)	Write an NDIS filter driver	Full kernel driver	😬 (very hard)
*/



//cat /sys/class/net/wlp7s0/type
//https://www.tcpdump.org/linktypes.html
//65534 - TUN

#[cfg(test)]
mod tests {
    use crate::capture::Capture;
    use crate::devices::Device;
    use crate::pcap::pcap::Pcap;
    use crate::pcap::pcapng::PcapNg;

    //#[test]
    //fn pcap() {
        //let pcap = Pcap::from_file("/home/brad/Downloads/sll2.pcap").expect("Failed to open pcap file");



        /*
        for packet in pcap {
            println!("{:?}", packet);
        }
        */

        //println!("{:?}", pcap);
    //}

    #[test]
    fn devices() {
        let devices = Device::list().unwrap();
        println!("{:?}", devices);
    }

    /*
    #[test]
    fn capture() {
        let devices = Device::list().unwrap();
        let device = devices.into_iter().find(|d| d.get_name().eq("Ethernet")).unwrap();
        let cap = Capture::from_device(&device).unwrap();
        cap.set_immediate_mode(true);
        cap.open().unwrap();

        loop {
        //for i in 0..10 {
            match cap.try_recv() {
                Ok((_, packet)) => {
                    println!("{:?}", packet);
                }
                Err(e) => {
                    //println!("{}", e.kind());
                }
            }
        }
    }*/
}
