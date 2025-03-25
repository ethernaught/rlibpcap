#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum IpVersions {
    Ipv4,
    Ipv6
}

impl IpVersions {

    pub fn from_code(code: u8) -> Result<Self, String> {
        for c in [Self::Ipv4, Self::Ipv6] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Self::Ipv4 => 4,
            Self::Ipv6 => 6
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Ipv4 => "IPv4",
            Self::Ipv6 => "IPv6"
        }.to_string()
    }
}
