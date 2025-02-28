use std::any::Any;
use std::net::Ipv6Addr;
use crate::packet::layers::ethernet_frame::ip::icmpv6::icmpv6_layer::Icmpv6Layer;
use crate::packet::layers::ethernet_frame::ip::inter::protocols::Protocols;
use crate::packet::layers::ethernet_frame::ip::tcp::tcp_layer::TcpLayer;
use crate::packet::layers::ethernet_frame::ip::udp::udp_layer::UdpLayer;
use crate::packet::layers::inter::layer::Layer;

const IPV6_HEADER_SIZE: usize = 40;

#[derive(Clone, Debug)]
pub struct Ipv6Layer {
    version: u8,
    traffic_class: u8,
    flow_label: u32,
    payload_length: u16,
    next_header: Protocols,
    hop_limit: u8,
    source_address: Ipv6Addr,
    destination_address: Ipv6Addr,
    data: Option<Box<dyn Layer>>
}

impl Ipv6Layer {

    pub fn get_version(&self) -> u8 {
        self.version
    }

    pub fn get_traffic_class(&self) -> u8 {
        self.traffic_class
    }

    pub fn get_flow_label(&self) -> u32 {
        self.flow_label
    }

    pub fn get_payload_length(&self) -> u16 {
        self.payload_length
    }

    pub fn get_next_header(&self) -> Protocols {
        self.next_header
    }

    pub fn get_hop_limit(&self) -> u8 {
        self.hop_limit
    }

    pub fn get_source_address(&self) -> &Ipv6Addr {
        &self.source_address
    }

    pub fn get_destination_address(&self) -> &Ipv6Addr {
        &self.destination_address
    }

    pub fn get_data(&self) -> Option<&Box<dyn Layer>> {
        self.data.as_ref()
    }

    pub fn get_data_mut(&mut self) -> Option<&mut Box<dyn Layer>> {
        self.data.as_mut()
    }
}

impl Layer for Ipv6Layer {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < IPV6_HEADER_SIZE {
            return None;
        }

        let next_header = Protocols::get_protocol_from_code(buf[6]).unwrap();

        let data = match next_header {
            Protocols::HopByHop => {
                None
            }
            Protocols::Icmp => {
                None
            }
            Protocols::Igmp => {
                None
            }
            Protocols::Tcp => {
                Some(TcpLayer::from_bytes(&buf[IPV6_HEADER_SIZE..])?.dyn_clone())
            }
            Protocols::Udp => {
                Some(UdpLayer::from_bytes(&buf[IPV6_HEADER_SIZE..])?.dyn_clone())
            }
            Protocols::Ipv6 => {
                None
            }
            Protocols::Gre => {
                None
            }
            Protocols::Icmpv6 => {
                Some(Icmpv6Layer::from_bytes(&buf[IPV6_HEADER_SIZE..])?.dyn_clone())
            }
            Protocols::Ospf => {
                None
            }
            Protocols::Sps => {
                None
            }
        };

        Some(Self {
            version: (buf[0] >> 4) & 0x0F,
            traffic_class: ((buf[0] & 0x0F) << 4) | (buf[1] >> 4),
            flow_label: ((buf[1] as u32 & 0x0F) << 16) | ((buf[2] as u32) << 8) | (buf[3] as u32),
            payload_length: u16::from_be_bytes([buf[4], buf[5]]),
            next_header,
            hop_limit: buf[7],
            source_address: Ipv6Addr::from(<[u8; 16]>::try_from(&buf[8..24]).unwrap()),
            destination_address: Ipv6Addr::from(<[u8; 16]>::try_from(&buf[24..40]).unwrap()),
            data
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; IPV6_HEADER_SIZE];

        buf[0] = (self.version << 4) | ((self.traffic_class >> 4) & 0x0F);
        buf[1] = ((self.traffic_class & 0x0F) << 4) | ((self.flow_label >> 16) as u8 & 0x0F);
        buf[2] = ((self.flow_label >> 8) & 0xFF) as u8;
        buf[3] = (self.flow_label & 0xFF) as u8;
        buf.splice(4..6, self.payload_length.to_be_bytes());
        buf[6] = self.next_header.get_code();
        buf[7] = self.hop_limit;
        buf.splice(8..24, self.source_address.octets());
        buf.splice(24..40, self.destination_address.octets());

        match &self.data {
            Some(data) => {
                buf.extend(data.to_bytes());
            }
            None => {}
        }

        buf
    }

    fn len(&self) -> usize {
        self.payload_length as usize + IPV6_HEADER_SIZE
    }

    fn compute_length(&mut self) -> usize {
        let payload_length = match &mut self.data {
            Some(layer) => {
                layer.len()
            }
            None => {
                0
            }
        };

        self.payload_length = payload_length as u16;
        payload_length + IPV6_HEADER_SIZE
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Layer> {
        Box::new(self.clone())
    }
}
