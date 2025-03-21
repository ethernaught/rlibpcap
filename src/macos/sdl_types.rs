use crate::packet::inter::data_link_types::DataLinkTypes;

impl DataLinkTypes {

    pub fn from_sdl_code(code: u8) -> Result<Self, String> {
        let known_types = [
            (0, Self::Null),
            (6, Self::En10mb),
            (18, Self::En10mb), //WAN... not sure
            (24, Self::En10mb)
        ];

        for (c, _type) in known_types {
            if c == code {
                return Ok(_type);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }
}
