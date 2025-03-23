use std::{io, mem};
use std::os::fd::RawFd;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::devices::Device;
use crate::macos::sys::SockAddrDl;
use crate::packet::inter::data_link_types::DataLinkTypes;
use crate::packet::packet::Packet;

#[derive(Debug, Clone)]
pub struct Capture {
    fd: RawFd,
    device: Option<Device>
}

impl Capture {

    pub fn any() -> io::Result<Self> {
        todo!()
    }

    pub fn from_device(device: &Device) -> io::Result<Self> {
        todo!()
    }

    pub fn open(&self) -> io::Result<()> {

        todo!()
    }

    pub fn set_immediate_mode(&self, immediate: bool) {
        println!("Setting capture to immediate mode");
    }

    pub fn send(&self, packet: Packet) -> io::Result<usize> {
        let mut packet = packet.to_bytes();

        todo!()
    }

    pub fn recv(&self) -> io::Result<(SockAddrDl, Packet)> { //i32 should be the socket address
        self.recv_with_flags(0)
    }

    pub fn try_recv(&self) -> io::Result<(SockAddrDl, Packet)> { //i32 should be the socket address
        self.recv_with_flags(0) //0 SHOULD BE RECEIVE ALL FLAG
    }

    fn recv_with_flags(&self, flags: i64) -> io::Result<(SockAddrDl, Packet)> { //i32 should be the socket address
        todo!()
    }

    pub fn close(&self) {
    }
}

/*
use std::ffi::{c_ulong, CString};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::mem;
use std::os::raw::{c_char, c_int, c_uchar};
use std::os::unix::io::AsRawFd;
use std::ptr;
use libc::ioctl;
use pcap::packet::inter::data_link_types::DataLinkTypes;
use pcap::packet::packet::Packet;

const BIOCSETIF: c_ulong = 0x8020426c;
const BIOCIMMEDIATE: c_ulong = 0x80044270;
const BIOCGBLEN: c_ulong = 0x40044266;
const DEFAULT_BPF_BUFFER_SIZE: usize = 4096;

#[repr(C)]
struct Ifreq {
    ifr_name: [c_char; 16],
    ifr_ifindex: c_int,
}

fn attach_interface(fd: c_int, iface: &str) -> Result<(), String> {
    let mut ifreq: Ifreq = unsafe { mem::zeroed() };
    let iface_cstr = CString::new(iface).map_err(|e| e.to_string())?;
    for (i, &byte) in iface_cstr.as_bytes().iter().enumerate() {
        ifreq.ifr_name[i] = byte as c_char;
    }

    let res = unsafe { ioctl(fd, BIOCSETIF, &ifreq) };
    if res < 0 {
        return Err("Failed to set interface".to_string());
    }
    Ok(())
}

fn set_immediate_mode(fd: c_int) -> Result<(), String> {
    let enable: c_int = 1;
    let res = unsafe { ioctl(fd, BIOCIMMEDIATE, &enable) };
    if res < 0 {
        return Err("Failed to set immediate mode".to_string());
    }
    Ok(())
}

fn get_buffer_len(fd: c_int) -> Result<usize, String> {
    let mut buf_len: c_int = 0;
    let res = unsafe { ioctl(fd, BIOCGBLEN, &mut buf_len) };
    if res < 0 {
        return Err("Failed to get buffer length".to_string());
    }
    Ok(buf_len as usize)
}

fn main() -> Result<(), String> {
    let bpf_path = "/dev/bpf0";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(bpf_path)
        .map_err(|e| format!("Failed to open {}: {}", bpf_path, e))?;

    let fd = file.as_raw_fd();

    let interface_name = "en0"; // Change to desired interface
    attach_interface(fd, interface_name)?;

    set_immediate_mode(fd)?;

    let buf_len = get_buffer_len(fd).unwrap_or(DEFAULT_BPF_BUFFER_SIZE);

    println!("Sniffing on interface: {}", interface_name);

    let mut buffer = vec![0u8; buf_len];

    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            continue;
        }

        let mut offset = 0;

        while offset + 18 <= n {
            let tstamp_sec = i32::from_ne_bytes(buffer[offset..offset + 4].try_into().unwrap());
            let tstamp_usec = i32::from_ne_bytes(buffer[offset + 4..offset + 8].try_into().unwrap());
            let caplen = u32::from_ne_bytes(buffer[offset + 8..offset + 12].try_into().unwrap());
            let datalen = u32::from_ne_bytes(buffer[offset + 12..offset + 16].try_into().unwrap());
            let hdrlen = u16::from_ne_bytes(buffer[offset + 16..offset + 18].try_into().unwrap());

            // Print parsed header for validation
            println!(
                "BpfHdr: tstamp_sec = {}, tstamp_usec = {}, caplen = {}, datalen = {}, hdrlen = {}",
                tstamp_sec, tstamp_usec, caplen, datalen, hdrlen
            );

            /*
            // Sanity check: Validate caplen and hdrlen
            if hdrlen < 18 || hdrlen as usize > 256 {
                println!("Invalid header length: {}. Skipping packet.", hdrlen);
                break;
            }

            if caplen == 0 || caplen as usize > n - offset - hdrlen as usize {
                println!("Invalid caplen: {}. Skipping packet.", caplen);
                break;
            }

            // Move to packet data after header
            let data_offset = offset + hdrlen as usize;
            if data_offset + caplen as usize > n {
                println!("Incomplete packet, skipping...");
                break;
            }
            */
            let data_offset = offset + hdrlen as usize;

            // Extract and print packet data
            let packet_data = &buffer[data_offset..(data_offset + caplen as usize)];
            println!("Packet Data (first 10 bytes): {:02X?}", &packet_data);

            //let packet = Packet::new(DataLinkTypes::En10mb, 0, &packet_data);
            //println!("{:?}", packet);

            // Move to the next packet
            offset += hdrlen as usize + caplen as usize;
        }
    }
}



//TODO
#[derive(Debug)]
struct BpfHdr {
    tstamp_sec: u32,     // Seconds from epoch
    tstamp_usec: u32,    // Microseconds
    caplen: u32,         // Length of captured portion
    datalen: u32,        // Original length of packet
    hdrlen: u16,         // Length of BPF header
}
*/
