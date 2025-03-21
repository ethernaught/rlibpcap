use crate::packet::inter::data_link_types::DataLinkTypes;

impl DataLinkTypes {

    pub fn from_sll2_code(code: u16) -> Result<Self, String> {
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
}
