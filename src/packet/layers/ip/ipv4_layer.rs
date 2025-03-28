use std::any::Any;
use std::net::Ipv4Addr;
use crate::packet::layers::ip::icmp::icmp_layer::IcmpLayer;
use crate::packet::layers::ip::inter::ip_protocols::IpProtocols;
use crate::packet::layers::ip::inter::ip_utils::calculate_checksum;
use crate::packet::layers::ip::inter::ip_versions::IpVersions;
use crate::packet::layers::ip::tcp::tcp_layer::TcpLayer;
use crate::packet::layers::ip::udp::udp_layer::UdpLayer;
use crate::packet::layers::inter::layer::Layer;

const IPV4_HEADER_LEN: usize = 20;

/*
let ihl = (packet[0] & 0x0F) as usize * 4; // Internet Header Length (IHL)
if ihl < IPV4_HEADER_SIZE || ihl > packet.len() {
    return Err(io::Error::new(io::ErrorKind::Other, "Packet has invalid IHL")); // Too short to be an IPv4 packet
}
*/

#[derive(Clone, Debug)]
pub struct Ipv4Layer {
    version: IpVersions,
    ihl: u8,
    tos: u8,
    total_length: u16,
    identification: u16,
    flags: u8,
    fragment_offset: u16,
    ttl: u8,
    protocol: IpProtocols,
    checksum: u16,
    source_address: Ipv4Addr,
    destination_address: Ipv4Addr,
    data: Option<Box<dyn Layer>>
}

impl Ipv4Layer {

    pub fn new(source_address: Ipv4Addr, destination_address: Ipv4Addr, protocol: IpProtocols) -> Self {
        Self {
            version: IpVersions::Ipv4,
            ihl: 5,
            tos: 0,
            total_length: IPV4_HEADER_LEN as u16,
            identification: 0,
            flags: 0,
            fragment_offset: 0,
            ttl: 64,
            protocol,
            checksum: 0,
            source_address,
            destination_address,
            data: None
        }
    }

    pub fn set_version(&mut self, version: IpVersions) {
        self.version = version;
    }

    pub fn get_version(&self) -> IpVersions {
        self.version
    }

    pub fn set_ihl(&mut self, ihl: u8) {
        self.ihl = ihl;
    }

    pub fn get_ihl(&self) -> u8 {
        self.ihl
    }

    pub fn set_tos(&mut self, tos: u8) {
        self.tos = tos;
    }

    pub fn get_tos(&self) -> u8 {
        self.tos
    }

    pub fn get_total_length(&self) -> u16 {
        self.total_length
    }

    pub fn set_identification(&mut self, identification: u16) {
        self.identification = identification;
    }

    pub fn get_identification(&self) -> u16 {
        self.identification
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn set_fragment_offset(&mut self, fragment_offset: u16) {
        self.fragment_offset = fragment_offset;
    }

    pub fn get_fragment_offset(&self) -> u16 {
        self.fragment_offset
    }

    pub fn set_ttl(&mut self, ttl: u8) {
        self.ttl = ttl;
    }

    pub fn get_ttl(&self) -> u8 {
        self.ttl
    }

    pub fn set_protocol(&mut self, protocol: IpProtocols) {
        self.protocol = protocol;
    }

    pub fn get_protocol(&self) -> IpProtocols {
        self.protocol
    }

    fn calculate_checksum(&self) -> u16 {
        let mut buf = vec![0; IPV4_HEADER_LEN];

        buf[0] = (self.version.get_code() << 4) | (self.ihl & 0x0F);
        buf[1] = self.tos;
        buf.splice(2..4, self.total_length.to_be_bytes());
        buf.splice(4..6, self.identification.to_be_bytes());
        buf[6] = (self.flags << 5) | ((self.fragment_offset >> 8) as u8 & 0x1F);
        buf[7] = (self.fragment_offset & 0xFF) as u8;
        buf[8] = self.ttl;
        buf[9] = self.protocol.get_code();
        buf.splice(12..16, self.source_address.octets());
        buf.splice(16..20, self.destination_address.octets());

        calculate_checksum(&buf)
    }

    pub fn compute_checksum(&mut self) -> u16 {
        let checksum = self.calculate_checksum();
        self.checksum = checksum;
        checksum
    }

    pub fn validate_checksum(&self) -> bool {
        self.checksum == self.calculate_checksum()
    }

    pub fn get_checksum(&self) -> u16 {
        self.checksum
    }

    pub fn set_source_address(&mut self, source_address: Ipv4Addr) {
        self.source_address = source_address;
    }

    pub fn get_source_address(&self) -> Ipv4Addr {
        self.source_address
    }

    pub fn set_destination_address(&mut self, destination_address: Ipv4Addr) {
        self.destination_address = destination_address;
    }

    pub fn get_destination_address(&self) -> Ipv4Addr {
        self.destination_address
    }

    pub fn set_data(&mut self, data: Box<dyn Layer>) {
        self.total_length = (data.len() + IPV4_HEADER_LEN) as u16;
        self.data = Some(data);
    }

    pub fn get_data(&self) -> Option<&Box<dyn Layer>> {
        self.data.as_ref()
    }

    pub fn get_data_mut(&mut self) -> Option<&mut Box<dyn Layer>> {
        self.data.as_mut()
    }
}

impl Layer for Ipv4Layer {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < IPV4_HEADER_LEN {
            return None;
        }

        let protocol = IpProtocols::from_code(buf[9]).unwrap();

        let data = match protocol {
            IpProtocols::HopByHop => {
                None
            }
            IpProtocols::Icmp => {
                Some(IcmpLayer::from_bytes(&buf[IPV4_HEADER_LEN..])?.dyn_clone())
            }
            IpProtocols::Igmp => {
                None
            }
            IpProtocols::Tcp => {
                Some(TcpLayer::from_bytes(&buf[IPV4_HEADER_LEN..])?.dyn_clone())
            }
            IpProtocols::Udp => {
                Some(UdpLayer::from_bytes(&buf[IPV4_HEADER_LEN..])?.dyn_clone())
            }
            IpProtocols::Ipv6 => {
                None
            }
            IpProtocols::Gre => {
                None
            }
            IpProtocols::Icmpv6 => {
                None
            }
            IpProtocols::Ospf => {
                None
            }
            IpProtocols::Sps => {
                None
            }
        };

        Some(Self {
            version: IpVersions::from_code((buf[0] >> 4) & 0x0F).unwrap(),
            ihl: buf[0] & 0x0F,
            tos: buf[1],
            total_length: u16::from_be_bytes([buf[2], buf[3]]),
            identification: u16::from_be_bytes([buf[4], buf[5]]),
            flags: buf[6] >> 5,
            fragment_offset: u16::from_be_bytes([buf[6] & 0x1F, buf[7]]),
            ttl: buf[8],
            protocol,
            checksum: u16::from_be_bytes([buf[10], buf[11]]),
            source_address: Ipv4Addr::new(buf[12], buf[13], buf[14], buf[15]),
            destination_address: Ipv4Addr::new(buf[16], buf[17], buf[18], buf[19]),
            data
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0; IPV4_HEADER_LEN];

        buf[0] = (self.version.get_code() << 4) | (self.ihl & 0x0F);
        buf[1] = self.tos;
        buf.splice(2..4, self.total_length.to_be_bytes());
        buf.splice(4..6, self.identification.to_be_bytes());
        buf[6] = (self.flags << 5) | ((self.fragment_offset >> 8) as u8 & 0x1F);
        buf[7] = (self.fragment_offset & 0xFF) as u8;
        buf[8] = self.ttl;
        buf[9] = self.protocol.get_code();
        buf.splice(10..12, self.checksum.to_be_bytes());
        buf.splice(12..16, self.source_address.octets());
        buf.splice(16..20, self.destination_address.octets());

        match &self.data {
            Some(data) => {
                buf.extend(data.to_bytes());
            }
            None => {}
        }

        buf
    }

    fn len(&self) -> usize {
        self.total_length as usize
    }

    fn compute_length(&mut self) -> usize {
        self.total_length = match &self.data {
            Some(layer) => {
                layer.len() + IPV4_HEADER_LEN
            }
            None => {
                IPV4_HEADER_LEN
            }
        } as u16;

        self.total_length as usize
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
