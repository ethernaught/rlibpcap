use crate::packet::layers::ethernet_frame::ip::udp::dhcp::dhcp_layer::DhcpLayer;
use crate::packet::layers::ethernet_frame::ip::udp::inter::udp_types::UdpTypes::Unknown;
use crate::packet::layers::inter::layer::Layer;

#[derive(Clone, Debug)]
pub enum UdpTypes {
    Dhcp(Box<dyn Layer>),
    Unknown(Vec<u8>)
}

impl UdpTypes {

    pub fn get_type_from_buf(buf: &[u8]) -> Self {
        let known_types = [
            ("DHCP", 236, [0x63, 0x82, 0x53, 0x63])
        ];

        for (variant, position, magic) in known_types {
            if buf.len() > position+magic.len() && &buf[position..position + magic.len()] == magic {
                return match variant {
                    "DHCP" => Self::Dhcp(DhcpLayer::from_bytes(&buf).unwrap().dyn_clone()),
                    _ => unreachable!()
                };
            }
        }

        Unknown(buf.to_vec())
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Dhcp(_) => "DHCP",
            _ => "UDP"
        }.to_string()
    }
}
