pub const PCAP_HEADER_LEN: usize = 24;
pub const MAGIC_NUMBER: u32 = 0xA1B2C3D4;

#[derive(Debug)]
pub struct Pcap {
    version_major: u16, // major version of the pcap file format
    version_minor: u16, // minor version
    thiszone: i32,      // GMT to local time correction
    sigfigs: u32,       // accuracy of timestamps
    snaplen: u32,       // maximum length of captured packets
    network: u32        // data link type
}

impl Pcap {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < PCAP_HEADER_LEN {
            return None;
        }

        if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != MAGIC_NUMBER {
            return None;
        }

        Some(Self {
            version_major: u16::from_le_bytes([buf[4], buf[5]]),
            version_minor: u16::from_le_bytes([buf[6], buf[7]]),
            thiszone: i32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
            sigfigs: u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]),
            snaplen: u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]),
            network: u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]),
        })
    }
}
