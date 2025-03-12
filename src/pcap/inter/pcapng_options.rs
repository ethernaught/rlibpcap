#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PcapNgOptions {
    End,
    Hardware,
    OperatingSystem,
    Application,
    /*
    DateAndTime,
    Hash,
    Comment,
    TimeZone,
    InterfaceName,
    CaptureFileId,
    PacketCount
    */
}

impl PcapNgOptions {

    pub fn from_code(code: u16) -> Result<Self, String> {
        for c in [
            Self::End,
            Self::Hardware,
            Self::OperatingSystem,
            Self::Application,
            /*
            Self::DateAndTime,
            Self::Hash,
            Self::Comment,
            Self::TimeZone,
            Self::InterfaceName,
            Self::CaptureFileId,
            Self::PacketCount
            */
        ] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u16 {
        match self {
            Self::End => 0,
            Self::Hardware => 2,
            Self::OperatingSystem => 3,
            Self::Application => 4,
            /*
            Self::DateAndTime => 4,         // Date and Time is Option Type 4
            Self::Hash => 5,
            Self::Comment => 6,
            Self::TimeZone => 7,
            Self::InterfaceName => 8,
            Self::CaptureFileId => 9,
            Self::PacketCount => 10
            */
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::End => "End",
            Self::Hardware => "Hardware",
            Self::OperatingSystem => "Operating System",
            Self::Application => "Application",
            /*
            Self::DateAndTime => "Date and Time",
            Self::Hash => "Hash",
            Self::Comment => "Comment",
            Self::TimeZone => "Time Zone",
            Self::InterfaceName => "Interface Name",
            Self::CaptureFileId => "Capture File ID",
            Self::PacketCount => "Packet Count"
            */
        }.to_string()
    }
}
