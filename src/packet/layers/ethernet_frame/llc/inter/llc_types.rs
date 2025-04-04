#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum LlcTypes {
    Ui,
    Test,
    Xid,
    Subme,
    Disc,
    Ua,
    Dm,
    Frmr,
    Sim,
    Rim
}

impl LlcTypes {

    pub fn from_code(code: u8) -> Result<Self, String> {
        for c in [Self::Ui, Self::Test, Self::Xid, Self::Subme, Self::Disc, Self::Ua, Self::Dm, Self::Frmr, Self::Sim, Self::Rim] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Self::Ui => 0x03,
            Self::Test => 0xE3,
            Self::Xid => 0xAF,
            Self::Subme => 0x6F,
            Self::Disc => 0x43,
            Self::Ua => 0x63,
            Self::Dm => 0x0F,
            Self::Frmr => 0x87,
            Self::Sim => 0x07,
            Self::Rim => 0x17
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Ui => "UI",
            Self::Test => "TEST",
            Self::Xid => "XID",
            Self::Subme => "SUBME",
            Self::Disc => "DISC",
            Self::Ua => "UA",
            Self::Dm => "DM",
            Self::Frmr => "FRMR",
            Self::Sim => "SIM",
            Self::Rim => "RIM"
        }.to_string()
    }
}
