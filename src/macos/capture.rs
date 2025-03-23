use std::{io, mem};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::devices::Device;
use crate::macos::sys::{ioctl, recvfrom, Ifreq, SockAddrDl, BIOCGBLEN, BIOCIMMEDIATE, BIOCSETIF, IFNAMSIZ};
use crate::packet::packet::Packet;

#[derive(Debug, Clone)]
pub struct Capture {
    fd: RawFd,
    device: Option<Device>,
    buffer: Vec<u8>,
    buf_len: usize,
    offset: usize
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
            buffer: vec![0u8; buf_len as usize],
            buf_len: buf_len as usize,
            offset: 0
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

    pub fn recv(&mut self) -> io::Result<(SockAddrDl, Packet)> { //i32 should be the socket address
        self.recv_with_flags(0)
    }

    pub fn try_recv(&mut self) -> io::Result<(SockAddrDl, Packet)> { //i32 should be the socket address
        self.recv_with_flags(0) //0 SHOULD BE RECEIVE ALL FLAG
    }

    fn recv_with_flags(&mut self, flags: i64) -> io::Result<(SockAddrDl, Packet)> { //i32 should be the socket address
        //let buf_len = get_buffer_len(self.fd).unwrap_or(DEFAULT_BPF_BUFFER_SIZE);

        if self.offset >= self.buf_len {
            // Refill the buffer with new data
            let n = unsafe { recvfrom(self.fd, self.buffer.as_mut_slice()) } as usize;
            if n == 0 {
                return Err(io::Error::new(io::ErrorKind::Other, "No packet received"));
            }
            self.offset = 0;  // Reset the offset after receiving new data
        }

        // Process the next packet in the buffer
        let n = self.buf_len;  // The size of the buffer we have to process

        // We process one packet at a time
        let mut packet_found = false;
        while self.offset + 18 <= n {
            let tstamp_sec = i32::from_ne_bytes(self.buffer[self.offset..self.offset + 4].try_into().unwrap());
            let tstamp_usec = i32::from_ne_bytes(self.buffer[self.offset + 4..self.offset + 8].try_into().unwrap());
            let caplen = u32::from_ne_bytes(self.buffer[self.offset + 8..self.offset + 12].try_into().unwrap());
            let datalen = u32::from_ne_bytes(self.buffer[self.offset + 12..self.offset + 16].try_into().unwrap());
            let hdrlen = u16::from_ne_bytes(self.buffer[self.offset + 16..self.offset + 18].try_into().unwrap());

            println!(
                "BpfHdr: tstamp_sec = {}, tstamp_usec = {}, caplen = {}, datalen = {}, hdrlen = {}",
                tstamp_sec, tstamp_usec, caplen, datalen, hdrlen
            );

            let data_offset = self.offset + hdrlen as usize;
            let packet_data = &self.buffer[data_offset..(data_offset + caplen as usize)];

            println!("Packet Data (first 10 bytes): {:02X?}", &packet_data);

            // Update offset for the next packet, ensuring the boundary alignment
            let total_len = hdrlen as usize + caplen as usize;
            self.offset += (total_len + 3) & !3; // Align to the next 4-byte boundary

            // Return the first valid packet and stop processing
            packet_found = true;
            let packet = Packet::new(packet_data.to_vec());
            let addr = SockAddrDl::default();  // Replace with actual address if needed
            return Ok((addr, packet));
        }

        // If no valid packet found, return an error
        if !packet_found {
            return Err(io::Error::new(io::ErrorKind::Other, "No valid packet found"));
        }

        // If we reach here, no valid packet was available
        Err(io::Error::new(io::ErrorKind::Other, "No valid packet found"))



        /*
        let mut buf_len: i64 = 0;
        let res = unsafe { ioctl(self.fd, BIOCGBLEN, &mut buf_len as *mut _ as i64) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut buffer = vec![0u8; buf_len as usize];

        loop {
            let n = unsafe { recvfrom(self.fd, buffer.as_mut_slice()) } as usize;
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

                println!(
                    "BpfHdr: tstamp_sec = {}, tstamp_usec = {}, caplen = {}, datalen = {}, hdrlen = {}",
                    tstamp_sec, tstamp_usec, caplen, datalen, hdrlen
                );

                let data_offset = offset + hdrlen as usize;

                let packet_data = &buffer[data_offset..(data_offset + caplen as usize)];
                println!("Packet Data (first 10 bytes): {:02X?}", &packet_data);

                let total_len = hdrlen as usize + caplen as usize;
                offset += (total_len + 3) & !3;
            }
        }
        */
    }

    pub fn close(&self) {
    }
}
