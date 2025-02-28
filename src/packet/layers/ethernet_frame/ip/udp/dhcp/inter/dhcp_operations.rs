#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DhcpOperations {
    BootRequest,
    BootReply
}

impl DhcpOperations {

    pub fn from_code(code: u8) -> Result<Self, String> {
        for c in [Self::BootRequest, Self::BootReply] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Self::BootRequest => 1,
            Self::BootReply => 2
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::BootRequest => "Request",
            Self::BootReply => "Reply"
        }.to_string()
    }
}
