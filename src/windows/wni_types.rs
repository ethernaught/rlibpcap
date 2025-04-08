use crate::utils::data_link_types::DataLinkTypes;

impl DataLinkTypes {

    pub fn from_wni_code(code: u32) -> Result<Self, String> {
        let known_types = [
            (1, Self::Null),
            (6, Self::En10mb),
            //(9, "Token Ring"), //TOKEN RING
            (23, Self::Ppp),
            (24, Self::Loop),
            (28, Self::Slip),
            (71, Self::Ieee802_11),
            (131, Self::Raw) //Tunnel
        ];

        for (c, _type) in known_types {
            if c == code {
                return Ok(_type);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }
}
