use crate::packet::layers::ethernet_frame::ip::udp::dhcp::dhcp_layer::DhcpLayer;
use crate::packet::layers::ethernet_frame::ip::udp::inter::udp_types::UdpTypes;
use crate::packet::layers::inter::layer::Layer;

#[derive(Clone, Debug)]
pub enum UdpPayloads {
    Known(UdpTypes, Box<dyn Layer>),
    Unknown(Vec<u8>),
    None
}

impl UdpPayloads {

    pub fn get_type_from_buf(buf: &[u8]) -> Self {
        let known_types = [
            (UdpTypes::Dhcp, 236, [0x63, 0x82, 0x53, 0x63])
        ];

        for (variant, position, magic) in known_types {
            if buf.len() > position+magic.len() && &buf[position..position + magic.len()] == magic {
                return match variant {
                    UdpTypes::Dhcp => Self::Known(variant, DhcpLayer::from_bytes(&buf).unwrap().dyn_clone()),
                    _ => unreachable!()
                };
            }
        }

        Self::Unknown(buf.to_vec())
    }
}
