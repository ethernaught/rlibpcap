#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DataLinkTypes {
    Null,
    Ethernet,
    Raw,
    Ipv4,
    Ipv6,
    BluetoothHciH4,
    Tun
}

impl DataLinkTypes {

    pub fn from_code(code: u32) -> Result<Self, String> {
        for c in [
            Self::Null,
            Self::Ethernet,
            Self::Raw,
            Self::Ipv4,
            Self::Ipv6,
            Self::BluetoothHciH4,
            Self::Tun
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
            Self::Ethernet => 1,
            Self::Raw => 13,
            Self::Ipv4 => 49,
            Self::Ipv6 => 50,
            Self::BluetoothHciH4 => 108,
            Self::Tun => 65534
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Null => "Null",
            Self::Ethernet => "Ethernet 10MB",
            Self::Raw => "Raw IP packets",
            Self::Ipv4 => "IPv4",
            Self::Ipv6 => "IPv6",
            Self::BluetoothHciH4 => "Bluetooth HCI",
            Self::Tun => "Tunnel"
        }.to_string()
    }
}
