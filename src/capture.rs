use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::devices::Device;
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::layer_1::ethernet_layer::EthernetLayer;
use crate::packet::layers::layer_1::inter::types::Types;
use crate::packet::layers::layer_2::ethernet::inter::protocols::Protocols;
use crate::packet::layers::layer_2::ethernet::ipv4_layer::IPv4Layer;
use crate::packet::layers::layer_2::ethernet::ipv6_layer::IPv6Layer;
use crate::packet::layers::layer_3::ip::tcp_layer::TcpLayer;
use crate::packet::layers::layer_3::ip::udp_layer::UdpLayer;
use crate::packet::packet::Packet;

const AF_PACKET: i64 = 17;
const SOCK_RAW: i64 = 3;
const ETH_P_ALL: u16 = 0x0003;
const SOL_SOCKET: i64 = 1;
const SO_BINDTODEVICE: i64 = 25;


#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct Capture {
    fd: RawFd,
    device: Device
}

#[cfg(target_os = "linux")]
use std::os::fd::RawFd;

#[cfg(target_os = "linux")]
impl Capture {

    pub fn from_device(device: Device) -> io::Result<Self> {
        let fd = unsafe {
            Self::syscall(41, AF_PACKET, SOCK_RAW, ETH_P_ALL.to_be() as i64, 0, 0)
        };

        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd: fd as RawFd,
            device
        })
    }

    pub fn open(&self) -> io::Result<()> {
        if self.fd < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut if_name_bytes = [0u8; 16];
        if let bytes = self.device.get_name().as_bytes() {
            if bytes.len() >= if_name_bytes.len() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
            }

            if_name_bytes[..bytes.len()].copy_from_slice(bytes);
        }

        let res = unsafe {
            Self::syscall(
                54,
                self.fd as i64,
                SOL_SOCKET,
                SO_BINDTODEVICE,
                if_name_bytes.as_ptr() as i64,
                if_name_bytes.len() as i64
            )
        };

        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    pub fn set_immediate_mode(&self, immediate: bool) {
        println!("Setting immediate mode for interface {}", self.device.get_name());
    }

    pub fn set_promiscuous_mode(&self, promiscuous: bool) {
        println!("Setting promiscuous mode for interface {}", self.device.get_name());
    }

    pub fn next_packet(&mut self) -> io::Result<Packet> {
        let mut buffer = vec![0u8; 4096];

        let len = unsafe {
            Self::syscall(
                45,
                self.fd as i64,
                buffer.as_mut_ptr() as i64,
                buffer.len() as i64,
                0,
                0
            )
        } as u32;

        if len > 0 {
            Ok(decode_packet(self.device.get_interface(), &buffer[..len as usize], len))

        } else {
            Err(io::Error::last_os_error())
        }
    }

    unsafe fn syscall(num: i64, a1: i64, a2: i64, a3: i64, a4: i64, a5: i64) -> i64 {
        let ret: i64;
        core::arch::asm!("syscall", in("rax") num, in("rdi") a1, in("rsi") a2, in("rdx") a3, in("r10") a4, in("r8") a5, lateout("rax") ret);
        ret
    }
}

fn decode_packet(interface: Interfaces, data: &[u8], len: u32) -> Packet {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    let mut frame = Packet::new(interface, now, len);

    match frame.get_interface() {
        Interfaces::Ethernet => {
            let ethernet_layer = EthernetLayer::from_bytes(&data).expect("Failed to parse Ethernet frame");
            frame.add_layer(ethernet_layer.dyn_clone());
            let mut off = ethernet_layer.len();

            match ethernet_layer.get_type() {
                Types::IPv4 => {
                    let ipv4_layer = IPv4Layer::from_bytes(&data[off..]).expect("Failed to parse IPv4 frame");
                    frame.add_layer(ipv4_layer.dyn_clone());
                    off += ipv4_layer.len();

                    match ipv4_layer.get_protocol() {
                        Protocols::Icmp => {}
                        Protocols::Igmp => {}
                        Protocols::Tcp => {
                            let tcp_layer = TcpLayer::from_bytes(&data[off..]).expect("Failed to parse TCP frame");
                            frame.add_layer(tcp_layer.dyn_clone());
                            off += tcp_layer.len();
                        }
                        Protocols::Udp => {
                            let udp_layer = UdpLayer::from_bytes(&data[off..]).expect("Failed to parse UDP frame");
                            frame.add_layer(udp_layer.dyn_clone());
                            off += udp_layer.len();
                        }
                        Protocols::Ipv6 => {}
                        Protocols::Icmpv6 => {}
                        Protocols::Gre => {}
                        Protocols::Ospf => {}
                        Protocols::Sps => {}
                    }




                }
                Types::Arp => {}
                Types::IPv6 => {
                    let ipv6_layer = IPv6Layer::from_bytes(&data[off..]).expect("Failed to parse IPv6 frame");
                    frame.add_layer(ipv6_layer.dyn_clone());
                    off += ipv6_layer.len();

                    match ipv6_layer.get_next_header() {
                        Protocols::Icmp => {}
                        Protocols::Igmp => {}
                        Protocols::Tcp => {
                            let tcp_layer = TcpLayer::from_bytes(&data[off..]).expect("Failed to parse TCP frame");
                            frame.add_layer(tcp_layer.dyn_clone());
                            off += tcp_layer.len();
                        }
                        Protocols::Udp => {
                            let udp_layer = UdpLayer::from_bytes(&data[off..]).expect("Failed to parse UDP frame");
                            frame.add_layer(udp_layer.dyn_clone());
                            off += udp_layer.len();
                        }
                        Protocols::Ipv6 => {}
                        Protocols::Icmpv6 => {}
                        Protocols::Gre => {}
                        Protocols::Ospf => {}
                        Protocols::Sps => {}
                    }
                }
                Types::Broadcast => {}
            }





        }
        Interfaces::WiFi => {}
        Interfaces::Bluetooth => {}
    }

    frame
}
