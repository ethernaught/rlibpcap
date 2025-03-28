#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DhcpMessageTypes {
    Discover,
    Offer,
    Request,
    Decline,
    Ack,
    Nak,
    Release,
    Inform
}

impl DhcpMessageTypes {

    pub fn from_code(code: u8) -> Result<Self, String> {
        for c in [Self::Discover, Self::Offer, Self::Request, Self::Decline, Self::Ack, Self::Nak, Self::Release, Self::Inform] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Self::Discover => 1,
            Self::Offer => 2,
            Self::Request => 3,
            Self::Decline => 4,
            Self::Ack => 5,
            Self::Nak => 6,
            Self::Release => 7,
            Self::Inform => 8
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Discover => "Discover",
            Self::Offer => "Offer",
            Self::Request => "Request",
            Self::Decline => "Decline",
            Self::Ack => "Pack",
            Self::Nak => "Nak",
            Self::Release => "Release",
            Self::Inform => "Inform"
        }.to_string()
    }
}
