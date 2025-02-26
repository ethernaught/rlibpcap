use std::net::Ipv4Addr;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use crate::packet::layers::inter::layer::Layer;

#[derive(Debug, Clone)]
pub struct Packet {
    interface: Interfaces,
    frame: Box<dyn Layer>,
    //layers: Vec<Box<dyn Layer>>,
    frame_time: u128,
    length: usize
}

impl Packet {

    pub fn new(interface: Interfaces, frame_time: u128, data: &[u8]) -> Self {
        let frame = match interface {
            Interfaces::Ethernet => {
                EthernetFrame::from_bytes(data).unwrap().dyn_clone()
            }
            Interfaces::WiFi => {
                todo!()
            }
            Interfaces::Bluetooth => {
                todo!()
            }
        };

        Self {
            interface,
            frame,
            //layers: Vec::new(),
            frame_time,
            length: data.len()
        }
    }

    pub fn get_interface(&self) -> &Interfaces {
        &self.interface
    }

    pub fn get_frame(&self) -> &Box<dyn Layer> {
        &self.frame
    }

    pub fn get_frame_time(&self) -> u128 {
        self.frame_time
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; self.length];
        /*
        let mut off = 0;

        for layer in &self.layers {
            let encoded_layer = layer.to_bytes();
            let len = encoded_layer.len();
            buf.splice(off..off + encoded_layer.len(), encoded_layer);
            off += len;
        }
        */

        buf
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

pub fn decode_packet(interface: Interfaces, data: &[u8]) -> Packet {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    let mut frame = Packet::new(interface, now, data);

    /*
    match frame.get_interface() {
        Interfaces::Ethernet => {
            let ethernet_layer = EthernetLayer::from_bytes(&data).expect("Failed to parse Ethernet frame");
            frame.add_layer(ethernet_layer.dyn_clone());
            let mut off = ethernet_layer.len();

            match ethernet_layer.get_type() {
                Types::IPv4 => {
                    let ipv4_layer = Ipv4Layer::from_bytes(&data[off..]).expect("Failed to parse IPv4 frame");
                    off += ipv4_layer.len();
                    frame.add_layer(ipv4_layer.dyn_clone());

                    match ipv4_layer.get_protocol() {
                        Protocols::HopByHop => {}
                        Protocols::Icmp => {
                            let icmp_layer = IcmpLayer::from_bytes(&data[off..]).expect("Failed to parse ICMP frame");
                            off += icmp_layer.len();
                            frame.add_layer(icmp_layer.dyn_clone());
                        }
                        Protocols::Igmp => {}
                        Protocols::Tcp => {
                            let mut tcp_layer = TcpLayer::from_bytes(&data[off..]).expect("Failed to parse TCP frame");
                            off += tcp_layer.len();
                            tcp_layer.set_payload(&data[off..]);

                            frame.add_layer(tcp_layer.dyn_clone());
                        }
                        Protocols::Udp => {
                            let mut udp_layer = UdpLayer::from_bytes(&data[off..]).expect("Failed to parse UDP frame");
                            off += udp_layer.len();

                            if ethernet_layer.get_destination().eq(&EthernetAddress::new(0xff, 0xff, 0xff, 0xff, 0xff, 0xff)) &&
                                    ipv4_layer.get_destination_ip().eq(&Ipv4Addr::new(255, 255, 255, 255)) &&
                                    udp_layer.get_destination_port() == 67 {
                                let dhcp_layer = DhcpLayer::from_bytes(&data[off..]).expect("Failed to parse DHCP frame");
                                frame.add_layer(dhcp_layer.dyn_clone());

                            } else {
                                udp_layer.set_payload(&data[off..]);
                            }

                            frame.add_layer(udp_layer.dyn_clone());
                        }
                        Protocols::Ipv6 => {}
                        Protocols::Icmpv6 => {}
                        Protocols::Gre => {}
                        Protocols::Ospf => {}
                        Protocols::Sps => {}
                    }




                }
                Types::Arp => {
                    let arp_layer = ArpLayer::from_bytes(&data[off..]).expect("Failed to parse ARP frame");
                    off += arp_layer.len();
                    frame.add_layer(arp_layer.dyn_clone());
                }
                Types::IPv6 => {
                    let ipv6_layer = Ipv6Layer::from_bytes(&data[off..]).expect("Failed to parse IPv6 frame");
                    off += ipv6_layer.len();
                    frame.add_layer(ipv6_layer.dyn_clone());

                    match ipv6_layer.get_next_header() {
                        Protocols::HopByHop => {}
                        Protocols::Icmp => {}
                        Protocols::Igmp => {}
                        Protocols::Tcp => {
                            let mut tcp_layer = TcpLayer::from_bytes(&data[off..]).expect("Failed to parse TCP frame");
                            off += tcp_layer.len();
                            tcp_layer.set_payload(&data[off..]);

                            frame.add_layer(tcp_layer.dyn_clone());
                        }
                        Protocols::Udp => {
                            let mut udp_layer = UdpLayer::from_bytes(&data[off..]).expect("Failed to parse UDP frame");
                            off += udp_layer.len();
                            udp_layer.set_payload(&data[off..]);

                            frame.add_layer(udp_layer.dyn_clone());
                        }
                        Protocols::Ipv6 => {}
                        Protocols::Icmpv6 => {
                            let icmpv6_layer = Icmpv6Layer::from_bytes(&data[off..]).expect("Failed to parse ICMPv6 frame");
                            off += icmpv6_layer.len();
                            frame.add_layer(icmpv6_layer.dyn_clone());
                        }
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
    */

    frame
}
