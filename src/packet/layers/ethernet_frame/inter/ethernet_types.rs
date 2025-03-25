#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EthernetTypes {
    Ipv4,
    Arp,
    Ipv6,
    Broadcast
}

impl EthernetTypes {

    pub fn from_code(code: u16) -> Result<Self, String> {
        for c in [Self::Ipv4, Self::Arp, Self::Ipv6, Self::Broadcast] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u16 {
        match self {
            Self::Ipv4 => 2048,
            Self::Arp => 2054,
            Self::Ipv6 => 34525,
            Self::Broadcast => 34969
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Ipv4 => "IPv4",
            Self::Arp => "ARP",
            Self::Ipv6 => "IPv6",
            Self::Broadcast => "Broadcast"
        }.to_string()
    }
}
