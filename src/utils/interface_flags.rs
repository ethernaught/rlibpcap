#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum InterfaceFlags {
    Up,
    Broadcast,
    Debug,
    Loopback,
    PointToPoint,
    NoTrailers,
    Running,
    NoArp,
    Promiscuous,
    AllMulti,
    Master,
    Slave,
    Multicast,
    Portsel,
    AutoMedia,
    Dynamic
}

impl InterfaceFlags {

    pub fn from_code(code: u32) -> Vec<Self> {
        let mut flags = Vec::new();

        for c in [
            Self::Up,
            Self::Broadcast,
            Self::Debug,
            Self::Loopback,
            Self::PointToPoint,
            Self::NoTrailers,
            Self::Running,
            Self::NoArp,
            Self::Promiscuous,
            Self::AllMulti,
            Self::Master,
            Self::Slave,
            Self::Multicast,
            Self::Portsel,
            Self::AutoMedia,
            Self::Dynamic
        ] {
            if code & c.get_code() != 0 {
                flags.push(c);
            }
        }

        flags
    }

    pub fn get_code(&self) -> u32 {
        match self {
            Self::Up => 0x1,
            Self::Broadcast => 0x2,
            Self::Debug => 0x4,
            Self::Loopback => 0x8,
            Self::PointToPoint => 0x10,
            Self::NoTrailers => 0x20,
            Self::Running => 0x40,
            Self::NoArp => 0x80,
            Self::Promiscuous => 0x100,
            Self::AllMulti => 0x200,
            Self::Master => 0x400,
            Self::Slave => 0x800,
            Self::Multicast => 0x1000,
            Self::Portsel => 0x2000,
            Self::AutoMedia => 0x4000,
            Self::Dynamic => 0x8000
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Up => "Up",
            Self::Broadcast => "Broadcast",
            Self::Debug => "Debug",
            Self::Loopback => "Loopback",
            Self::PointToPoint => "Point to Point",
            Self::NoTrailers => "No Trailers",
            Self::Running => "Running",
            Self::NoArp => "No Arp",
            Self::Promiscuous => "Promiscuous",
            Self::AllMulti => "All Multi",
            Self::Master => "Master",
            Self::Slave => "Slave",
            Self::Multicast => "Multicast",
            Self::Portsel => "Portsel",
            Self::AutoMedia => "Auto Media",
            Self::Dynamic => "Dynamic"
        }.to_string()
    }
}
