#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PacketTypes {
    Host,
    Broadcast,
    Multicast,
    OtherHost,
    OutGoing,
    LoopBack
}

impl PacketTypes {

    pub fn from_code(code: u16) -> Result<Self, String> {
        for c in [Self::Host, Self::Broadcast, Self::Multicast, Self::OtherHost, Self::OutGoing, Self::LoopBack] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u16 {
        match self {
            Self::Host => 0,
            Self::Broadcast => 1,
            Self::Multicast => 2,
            Self::OtherHost => 3,
            Self::OutGoing => 4,
            Self::LoopBack => 6
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Host => "PACKET_HOST",
            Self::Broadcast => "PACKET_BROADCAST",
            Self::Multicast => "PACKET_MULTICAST",
            Self::OtherHost => "PACKET_OTHERHOST",
            Self::OutGoing => "PACKET_OUTGOING",
            Self::LoopBack => "PACKET_LOOPBACK"
        }.to_string()
    }
}
