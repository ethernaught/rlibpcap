#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LoopTypes {
    Ipv4,
    Osi,
    Ipx,
    Ipv6,
    Ipv6e2,
    Ipv6e3
}

impl LoopTypes {

    pub fn from_code(code: u32) -> Result<Self, String> {
        for c in [Self::Ipv4, Self::Osi, Self::Ipx, Self::Ipv6, Self::Ipv6e2, Self::Ipv6e3] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u32 {
        match self {
            Self::Ipv4 => 2,
            Self::Osi => 7,
            Self::Ipx => 23,
            Self::Ipv6 => 24,
            Self::Ipv6e2 => 28,
            Self::Ipv6e3 => 30
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Ipv4 => "IPv4",
            Self::Osi => "OSI",
            Self::Ipx => "IPX",
            Self::Ipv6 => "IPv6",
            Self::Ipv6e2 => "IPv6",
            Self::Ipv6e3 => "IPv6"
        }.to_string()
    }
}
