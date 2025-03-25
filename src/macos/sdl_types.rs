use crate::utils::data_link_types::DataLinkTypes;

impl DataLinkTypes {

    pub fn from_sdl_code(code: u8) -> Result<Self, String> {
        let known_types = [
            (1, Self::Null),
            (6, Self::En10mb),
            (18, Self::Slip), //Slip
            (28, Self::Ppp), //PPP
            (24, Self::Loop),
            (55, Self::Raw), //IPSec VPN
            (57, Self::Raw), //IP TUNNEL
            (108, Self::Ieee802), //WiFi
            //(114, Self::Vlan), //VLAN
            //(131, "Bridge"), //BRIDGE
            (135, Self::Raw) //Tunnel
        ];

        for (c, _type) in known_types {
            if c == code {
                return Ok(_type);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }
}
