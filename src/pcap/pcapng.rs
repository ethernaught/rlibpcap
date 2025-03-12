use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::vec::IntoIter;
use crate::packet::inter::data_link_types::DataLinkTypes;
use crate::packet::packet::Packet;
use crate::pcap::inter::pcapng_options::PcapNgOptions;

pub const PCAP_HEADER_LEN: usize = 24;
pub const MAGIC_NUMBER: u32 = 0x0A0D0D0A;
pub const BYTE_ORDER_MAGIC: u32 = 0x1A2B3C4D; // Indicates Little Endian

#[derive(Debug, Clone)]
pub struct PcapNg {
    byte_order: bool,  // true = Little Endian, false = Big Endian
    version_major: u16,
    version_minor: u16,
    section_length: i64, // Can be -1 if unspecified
    options: Vec<u8>, // Store raw options for now
    data_link_type: DataLinkTypes,
    packets: Vec<Packet>
}

impl PcapNg {

    /*
    pub fn new() -> Self {
        Self {
            version_major: 2, //VERIFY...
            version_minor: 5, //VERIFY...
            zone: 0,
            accuracy: 0, //I THINK 0...
            payload_length: 0,
            data_link_type: DataLinkTypes::Null,
            packets: Vec::new()
        }
    }
    */

    pub fn from_file(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut buf = [0u8; 24];
        file.read_exact(&mut buf)?;

        if buf.len() < PCAP_HEADER_LEN {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid header length in pcap file"));
        }

        if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != MAGIC_NUMBER {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Magic number mismatch"));
        }





        let block_length = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
        if block_length < 24 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid SHB block length"));
        }

        let byte_order_magic = u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);
        let byte_order = byte_order_magic == BYTE_ORDER_MAGIC;

        let (version_major, version_minor, section_length) = if byte_order {
            (
                u16::from_le_bytes([buf[12], buf[13]]),
                u16::from_le_bytes([buf[14], buf[15]]),
                i64::from_le_bytes([buf[16], buf[17], buf[18], buf[19], buf[20], buf[21], buf[22], buf[23]]),
            )
        } else {
            (
                u16::from_be_bytes([buf[12], buf[13]]),
                u16::from_be_bytes([buf[14], buf[15]]),
                i64::from_be_bytes([buf[16], buf[17], buf[18], buf[19], buf[20], buf[21], buf[22], buf[23]]),
            )
        };

        let options_length = (block_length - 24 - 4) as usize;
        let mut buf = vec![0u8; options_length];
        file.read_exact(&mut buf)?;


        let mut off = 0;

        while off < options_length {
            let opt = PcapNgOptions::from_code(u16::from_le_bytes([buf[off], buf[off+1]]))
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            if opt == PcapNgOptions::End {
                break;
            }

            let length = u16::from_le_bytes([buf[off+2], buf[off+3]]) as usize;

            let value = String::from_utf8_lossy(&buf[off + 4..off + 4 + length]);

            println!("{} {} {:?}", opt.to_string(), length, value);

            let padding = (4 - ((length + 4) % 4)) % 4;
            off += padding + length + 4;
        }

        //let opt = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);

        /*
        let options_length = (block_length - 24 - 4) as usize;
        let mut options = vec![0u8; options_length];
        file.read_exact(&mut options)?;

        let mut final_block_length_buf = [0u8; 4];
        file.read_exact(&mut final_block_length_buf)?;
        let final_block_length = u32::from_le_bytes(final_block_length_buf);

        if final_block_length != block_length {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Mismatched SHB block length"));
        }
        */





        Ok(Self {
            byte_order,
            version_major,
            version_minor,
            section_length,
            options: buf,
            data_link_type: DataLinkTypes::Null,
            packets: Vec::new()
        })
    }
}

impl IntoIterator for PcapNg {

    type Item = Packet;
    type IntoIter = IntoIter<Packet>;

    fn into_iter(self) -> Self::IntoIter {
        self.packets.into_iter()
    }
}
