use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use std::vec::IntoIter;
use crate::packet::inter::interfaces::Interfaces;
use crate::packet::packet::Packet;
use crate::pcap::inter::pcap_networks::PcapNetworks;

pub const PCAP_HEADER_LEN: usize = 24;
pub const MAGIC_NUMBER: u32 = 0xA1B2C3D4;

#[derive(Debug)]
pub struct Pcap {
    version_major: u16,
    version_minor: u16,
    zone: i32,
    accuracy: u32,
    payload_length: u32,
    network: PcapNetworks,
    packets: Vec<Packet>
}

#[derive(Debug)]
pub struct PacketHeader {
    timestamp_sec: u32,       // Timestamp seconds
    timestamp_usec: u32,      // Timestamp microseconds
    captured_len: u32,        // Length of captured packet
    original_len: u32,        // Original packet length
}

impl Pcap {

    pub fn from_file(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut buf = [0u8; 24]; // Global header is 24 bytes
        file.read_exact(&mut buf)?;

        if buf.len() < PCAP_HEADER_LEN {
            return Err(io::Error::new(ErrorKind::InvalidData, "Invalid header length in pcap file"));
        }

        if u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) != MAGIC_NUMBER {
            return Err(io::Error::new(ErrorKind::InvalidData, "Magic number mismatch"));
        }

        let version_major = u16::from_le_bytes([buf[4], buf[5]]);
        let version_minor = u16::from_le_bytes([buf[6], buf[7]]);
        let zone = i32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]);
        let accuracy = u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]);
        let payload_length = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]);
        let network = PcapNetworks::from_code(u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]))?;

        let mut packets = Vec::new();

        loop {
            let mut buf = [0u8; 16];
            if file.read_exact(&mut buf).is_err() {
                break;
            }

            let packet_header = PacketHeader {
                timestamp_sec: u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
                timestamp_usec: u32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]),
                captured_len: u32::from_le_bytes([buf[8], buf[9], buf[10], buf[11]]),
                original_len: u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]),
            };

            let mut buf = vec![0u8; packet_header.captured_len as usize];
            file.read_exact(&mut buf)?;

            packets.push(Packet::new(Interfaces::Ethernet, 0, &buf));
        }

        Ok(Self {
            version_major,
            version_minor,
            zone,
            accuracy,
            payload_length,
            network,
            packets
        })
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

    pub fn get_payload_length(&self) -> usize {
        self.payload_length as usize
    }

    pub fn add_packet(&mut self, packet: Packet) {
        self.packets.push(packet);
    }

    pub fn get_packet(&self, index: usize) -> &Packet {
        &self.packets[index]
    }

    pub fn total_packets(&self) -> usize {
        self.packets.len()
    }
}

impl IntoIterator for Pcap {

    type Item = Packet;
    type IntoIter = IntoIter<Packet>;

    fn into_iter(self) -> Self::IntoIter {
        self.packets.into_iter()
    }
}
