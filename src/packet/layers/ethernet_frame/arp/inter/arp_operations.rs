#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ArpOperations {
    Request,
    Reply,
    Rarp,
    Nak,
    InArp
}

impl ArpOperations {

    pub fn from_code(code: u16) -> Result<Self, String> {
        for c in [Self::Request, Self::Reply, Self::Rarp, Self::Nak, Self::InArp] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u16 {
        match self {
            Self::Request => 1,
            Self::Reply => 2,
            Self::Rarp => 3,
            Self::Nak => 4,
            Self::InArp => 15
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Request => "ARP Request",
            Self::Reply => "ARP Reply",
            Self::Rarp => "RARP Request",
            Self::Nak => "ARP-NAK",
            Self::InArp => "InARP Request"
        }.to_string()
    }
}
