use std::{io, mem};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::devices::Device;
use crate::macos::sys::{ioctl, recvfrom, Ifreq, SockAddrDl, BIOCGBLEN, BIOCIMMEDIATE, BIOCSETIF, IFNAMSIZ};
use crate::packet::inter::data_link_types::DataLinkTypes;
use crate::packet::packet::Packet;

#[derive(Debug, Clone)]
pub struct Capture {
    fd: RawFd,
    device: Option<Device>,
    packet_buffer: Vec<Packet>
}

impl Capture {

    pub fn any() -> io::Result<Self> {
        let bpf_path = "/dev/bpf0";
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(bpf_path)
            .unwrap();
            //.map_err(|e| io::Error(format!("Failed to open {}: {}", bpf_path, e)))?;

        let fd = file.into_raw_fd();

        let mut buf_len: i64 = 0;
        let res = unsafe { ioctl(fd, BIOCGBLEN, &mut buf_len as *mut _ as i64) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd,
            device: None,
            packet_buffer: Vec::new()
        })
    }

    pub fn from_device(device: &Device) -> io::Result<Self> {
        /*
        Ok(Self {
            fd,
            device: None,
        })
        */
        todo!()
    }

    pub fn open(&self) -> io::Result<()> {
        let interface_name = "en0";

        let mut ifreq = Ifreq {
            ifr_name: [0; IFNAMSIZ],
            ifr_ifindex: 0,
        };

        let if_name_bytes = interface_name.as_bytes().to_vec();
        //let if_name_bytes = device.get_name().into_bytes();
        if if_name_bytes.len() >= IFNAMSIZ {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
        }

        ifreq.ifr_name[..if_name_bytes.len()].copy_from_slice(&if_name_bytes);

        let res = unsafe { ioctl(self.fd, BIOCSETIF, &ifreq as *const _ as i64) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        println!("Sniffing on interface: {}", interface_name);
        self.set_immediate_mode(true)?;

        Ok(())
    }

    pub fn set_immediate_mode(&self, immediate: bool) -> io::Result<()> {
        let enable: i64 = 1;
        let res = unsafe { ioctl(self.fd, BIOCIMMEDIATE, &enable as *const _ as i64) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub fn send(&self, packet: Packet) -> io::Result<usize> {
        let mut packet = packet.to_bytes();

        todo!()
    }

    pub fn recv(&mut self) -> io::Result<(i32, Packet)> { //i32 should be the socket address
        self.recv_with_flags(0)
    }

    pub fn try_recv(&mut self) -> io::Result<(i32, Packet)> { //i32 should be the socket address
        self.recv_with_flags(0) //0 SHOULD BE RECEIVE ALL FLAG
    }

    fn recv_with_flags(&mut self, flags: i64) -> io::Result<(i32, Packet)> { //i32 should be the socket address
        //let buf_len = get_buffer_len(self.fd).unwrap_or(DEFAULT_BPF_BUFFER_SIZE);
        if !self.packet_buffer.is_empty() {
            return Ok((0, self.packet_buffer.remove(0)));
        }

        let mut buf_len: i64 = 0;
        let res = unsafe { ioctl(self.fd, BIOCGBLEN, &mut buf_len as *mut _ as i64) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut buffer = vec![0u8; buf_len as usize];

        let len = unsafe { recvfrom(self.fd, buffer.as_mut_slice()) } as isize;
        if len > 0 {
            let mut offset = 0 as usize;

            while offset + 18 <= len as usize {
                let tstamp_sec = i32::from_ne_bytes(buffer[offset..offset + 4].try_into().unwrap());
                let tstamp_usec = i32::from_ne_bytes(buffer[offset + 4..offset + 8].try_into().unwrap());
                let caplen = u32::from_ne_bytes(buffer[offset + 8..offset + 12].try_into().unwrap());
                let datalen = u32::from_ne_bytes(buffer[offset + 12..offset + 16].try_into().unwrap());
                let hdrlen = u16::from_ne_bytes(buffer[offset + 16..offset + 18].try_into().unwrap());

                println!(
                    "BpfHdr: tstamp_sec = {}, tstamp_usec = {}, caplen = {}, datalen = {}, hdrlen = {}",
                    tstamp_sec, tstamp_usec, caplen, datalen, hdrlen
                );

                let data_offset = offset + hdrlen as usize;

                let packet_data = &buffer[data_offset..(data_offset + caplen as usize)];
                //println!("Packet Data (first 10 bytes): {:02X?}", &packet_data);

                let packet = Packet::new(DataLinkTypes::En10mb, 0, packet_data);

                self.packet_buffer.push(packet);


                let total_len = hdrlen as usize + caplen as usize;
                offset += (total_len + 3) & !3;
            }

            return Ok((0, self.packet_buffer.remove(0)));

        } else if len == -1 {
            let err = io::Error::last_os_error();

            if err.kind() == io::ErrorKind::WouldBlock {
                return Err(io::Error::new(io::ErrorKind::WouldBlock, "No data available"));
            }

            return Err(err);
        }

        Err(io::Error::last_os_error())
    }

    pub fn close(&self) {
    }
}
