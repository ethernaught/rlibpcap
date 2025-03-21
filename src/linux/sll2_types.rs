use crate::packet::inter::data_link_types::DataLinkTypes;

impl DataLinkTypes {

    pub fn from_arphrd_code(code: u16) -> Result<Self, String> {
        let known_types = [
            (0, Self::Null),
            (1, Self::En10mb),
            (12, Self::Raw),
            (49, Self::Ipv4),
            (50, Self::Ipv6),
            //(108, Self::Bluetooth),
            //(276, Self::Raw), - Sll2
            (772, Self::En10mb),
            (65534, Self::Raw)

        ];

        for (c, _type) in known_types {
            if c == code {
                return Ok(_type);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_arphrd_code(&self) -> u16 {
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
            Self::Raw => 12, //14 for BSD
        }
    }
}
