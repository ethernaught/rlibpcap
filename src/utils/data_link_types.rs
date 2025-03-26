#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DataLinkTypes {
    Null,
    En10mb,
    En3mb,
    Ax25,
    ProNet,
    Chaos,
    Ieee802,
    ArcNet,
    Slip,
    Ppp,
    Fddi,
    Raw,
    Loop,
    Ipv4,
    Ipv6,
    Sll2
}

impl DataLinkTypes {

    pub fn from_code(code: u32) -> Result<Self, String> {
        for c in [
            Self::Null,
            Self::En10mb,
            Self::En3mb,
            Self::Ax25,
            Self::ProNet,
            Self::Chaos,
            Self::Ieee802,
            Self::ArcNet,
            Self::Slip,
            Self::Ppp,
            Self::Fddi,
            Self::Raw,
            Self::Loop,
            Self::Ipv4,
            Self::Ipv6,
            Self::Sll2
        ] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u32 {
        match self {
            Self::Null => 0,
            Self::En10mb => 1,
            Self::En3mb => 2,
            Self::Ax25 => 3,
            Self::ProNet => 4,
            Self::Chaos => 5,
            Self::Ieee802 => 6,
            Self::ArcNet => 7,
            Self::Slip => 8,
            Self::Ppp => 9,
            Self::Fddi => 10,
            Self::Raw => 101,
            Self::Loop => 108,
            Self::Ipv4 => 228,
            Self::Ipv6 => 229,
            Self::Sll2 => 276
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Null => "Null",
            Self::En10mb => "Ethernet",
            Self::En3mb => "Experimental Ethernet",
            Self::Ax25 => "AX.25",
            Self::ProNet => "ProNET Token Ring",
            Self::Chaos => "Chaosnet",
            Self::Ieee802 => "IEEE802",
            Self::ArcNet => "ARCNET",
            Self::Slip => "SLIP",
            Self::Ppp => "PPP",
            Self::Fddi => "FDDI",
            Self::Raw => "Raw",
            Self::Loop => "Loop",
            Self::Ipv4 => "IPv4",
            Self::Ipv6 => "IPv6",
            Self::Sll2 => "SLL2"
        }.to_string()
    }
}
