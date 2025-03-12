use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::vec::IntoIter;
use crate::packet::inter::data_link_types::DataLinkTypes;
use crate::packet::packet::Packet;

pub const PCAP_HEADER_LEN: usize = 24;
pub const MAGIC_NUMBER: u32 = 0xA1B2C3D4;

#[derive(Debug, Clone)]
pub struct Pcap {
    version_major: u16,
    version_minor: u16,
    zone: i32,
    accuracy: u32,
    payload_length: u32,
    data_link_type: DataLinkTypes,
    packets: Vec<Packet>
}

impl Pcap {

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

        let version_major = u16::from_le_bytes([buf[4], buf[5]]);
        let version_minor = u16::from_le_bytes([buf[6], buf[7]]);
        let zone = i32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);
        let accuracy = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
        let payload_length = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]);
        let data_link_type = DataLinkTypes::from_code(u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut packets = Vec::new();

        loop {
            let mut buf = [0u8; 16];
            if file.read_exact(&mut buf).is_err() {
                break;
            }

            let timestamp_sec = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
            let timestamp_usec = u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
            let captured_len = u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);
            let frame_time = Self::combine_timestamp(timestamp_sec, timestamp_usec);

            let mut buf = vec![0u8; captured_len as usize];
            file.read_exact(&mut buf)?;

            packets.push(Packet::new(data_link_type, frame_time, &buf));
        }

        Ok(Self {
            version_major,
            version_minor,
            zone,
            accuracy,
            payload_length,
            data_link_type,
            packets
        })
    }

    pub fn to_file(&self, file_path: &str) -> io::Result<()> {
        let mut file = File::create(file_path)?;

        file.write_all(&MAGIC_NUMBER.to_le_bytes())?;
        file.write_all(&self.version_major.to_le_bytes())?;
        file.write_all(&self.version_minor.to_le_bytes())?;
        file.write_all(&self.zone.to_le_bytes())?;
        file.write_all(&self.accuracy.to_le_bytes())?;
        file.write_all(&self.payload_length.to_le_bytes())?;
        file.write_all(&self.data_link_type.get_code().to_le_bytes())?;

        for packet in &self.packets {
            let (timestamp_sec, timestamp_usec) = Self::split_timestamp(packet.get_frame_time());
            file.write_all(&timestamp_sec.to_le_bytes())?;
            file.write_all(&timestamp_usec.to_le_bytes())?;
            file.write_all(&packet.len().to_le_bytes())?;
            file.write_all(&packet.len().to_le_bytes())?;

            file.write_all(&packet.to_bytes())?;
        }

        Ok(())
    }

    pub fn get_version_major(&self) -> u16 {
        self.version_major
    }

    pub fn get_version_minor(&self) -> u16 {
        self.version_major
    }

    pub fn get_zone(&self) -> i32 {
        self.zone
    }

    pub fn get_accuracy(&self) -> u32 {
        self.accuracy
    }

    pub fn compute_length(&self) -> usize {
        let mut payload_length = 0;
        for packet in &self.packets {
            payload_length += packet.len();
        }

        self.payload_length as usize
    }

    pub fn get_payload_length(&self) -> usize {
        self.payload_length as usize
    }

    pub fn get_data_link_type(&self) -> DataLinkTypes {
        self.data_link_type
    }

    pub fn set_packets(&mut self, packets: Vec<Packet>) {
        self.packets = packets;
    }

    pub fn add_packet(&mut self, packet: Packet) {
        self.packets.push(packet);
    }

    pub fn get_packet(&self, index: usize) -> &Packet {
        &self.packets[index]
    }

    pub fn get_packets(&self) -> Vec<Packet> {
        self.packets.clone()
    }

    pub fn total_packets(&self) -> usize {
        self.packets.len()
    }

    fn split_timestamp(timestamp_ns: u128) -> (u32, u32) {
        let timestamp_sec = (timestamp_ns / 1_000_000_000) as u32;
        let timestamp_usec = ((timestamp_ns % 1_000_000_000) / 1_000) as u32;
        (timestamp_sec, timestamp_usec)
    }

    fn combine_timestamp(timestamp_sec: u32, timestamp_usec: u32) -> u128 {
        let timestamp_ns = (timestamp_sec as u128 * 1_000_000_000) + (timestamp_usec as u128 * 1_000);
        timestamp_ns
    }
}

impl IntoIterator for Pcap {

    type Item = Packet;
    type IntoIter = IntoIter<Packet>;

    fn into_iter(self) -> Self::IntoIter {
        self.packets.into_iter()
    }
}
