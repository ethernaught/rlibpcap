#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PcapNetworks {
    Null,
    Ethernet,
    IEEE802,
    Arcnet,
    Slip,
    Ppp,
    Fddi,
    AtmRfc1483,
    Raw,
    SlipBsdos,
    PppBsdos,
    Mpls,
    Hdlc,
    LinuxSll,
    LocalTalk,
    PfLog,
    Ieee80211,
    Arp,
    Vlan,
    MplsMcast,
    Can20b,
    Ipv4,
    Ipv6,
    BluetoothHciH4
}

impl PcapNetworks {

    pub fn from_code(code: u32) -> Result<Self, String> {
        for c in [
            Self::Null,
            Self::Ethernet,
            Self::IEEE802,
            Self::Arcnet,
            Self::Slip,
            Self::Ppp,
            Self::Fddi,
            Self::AtmRfc1483,
            Self::Raw,
            Self::SlipBsdos,
            Self::PppBsdos,
            Self::Mpls,
            Self::Hdlc,
            Self::LinuxSll,
            Self::LocalTalk,
            Self::PfLog,
            Self::Ieee80211,
            Self::Arp,
            Self::Vlan,
            Self::MplsMcast,
            Self::Can20b,
            Self::Ipv4,
            Self::Ipv6,
            Self::BluetoothHciH4
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
            Self::IEEE802 => 6,
            Self::Arcnet => 7,
            Self::Slip => 8,
            Self::Ppp => 9,
            Self::Fddi => 11,
            Self::AtmRfc1483 => 12,
            Self::Raw => 13,
            Self::SlipBsdos => 14,
            Self::PppBsdos => 15,
            Self::Mpls => 33,
            Self::Hdlc => 34,
            Self::LinuxSll => 35,
            Self::LocalTalk => 36,
            Self::PfLog => 37,
            Self::Ieee80211 => 40,
            Self::Arp => 42,
            Self::Vlan => 43,
            Self::MplsMcast => 44,
            Self::Can20b => 48,
            Self::Ipv4 => 49,
            Self::Ipv6 => 50,
            Self::BluetoothHciH4 => 108
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Null => "",
            Self::Ethernet => "",
            Self::IEEE802 => "",
            Self::Arcnet => "",
            Self::Slip => "",
            Self::Ppp => "",
            Self::Fddi => "",
            Self::AtmRfc1483 => "",
            Self::Raw => "",
            Self::SlipBsdos => "",
            Self::PppBsdos => "",
            Self::Mpls => "",
            Self::Hdlc => "",
            Self::LinuxSll => "",
            Self::LocalTalk => "",
            Self::PfLog => "",
            Self::Ieee80211 => "",
            Self::Arp => "",
            Self::Vlan => "",
            Self::MplsMcast => "",
            Self::Can20b => "",
            Self::Ipv4 => "",
            Self::Ipv6 => "",
            Self::BluetoothHciH4 => ""
        }.to_string()
    }
}
