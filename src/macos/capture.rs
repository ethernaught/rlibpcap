use std::{io, ptr};
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::Read;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use crate::devices::Device;
use crate::macos::sys::{ioctl, recvfrom, select, TimeVal, Ifreq, BIOCGBLEN, BIOCIMMEDIATE, BIOCSETIF, IFNAMSIZ, close};
use crate::utils::data_link_types::DataLinkTypes;
use crate::packet::packet::Packet;

#[derive(Debug, Clone)]
pub struct Capture {
    fd: RawFd,
    device: Option<Device>,
    packet_buffer: RefCell<Vec<Packet>>,
    buffer_len: usize
}

impl Capture {

    pub fn from_device(device: &Device) -> io::Result<Self> {

        //DYNAMICALLY KNOW MAX BPF DEVICES...

        let mut file = None;
        for i in 0..256 {
            let f = OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!("/dev/bpf{}", i));

            if f.is_ok() {
                file = Some(f.unwrap());
                break;
            }
        }

        let fd = file.unwrap().into_raw_fd();

        let mut buf_len = 0;
        if unsafe { ioctl(fd, BIOCGBLEN, &mut buf_len as *mut _ as i64) } < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            fd,
            device: Some(device.clone()),
            packet_buffer: RefCell::new(Vec::new()),
            buffer_len: buf_len
        })
    }

    pub fn open(&self) -> io::Result<()> {
        match self.device {
            Some(ref device) => {
                let mut ifreq = Ifreq {
                    ifr_name: [0; IFNAMSIZ],
                    ifr_ifindex: 0,
                };

                let if_name_bytes = device.get_name().into_bytes();
                if if_name_bytes.len() >= IFNAMSIZ {
                    unsafe { close(self.fd) };
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Interface name too long"));
                }

                ifreq.ifr_name[..if_name_bytes.len()].copy_from_slice(&if_name_bytes);

                if unsafe { ioctl(self.fd, BIOCSETIF, &ifreq as *const _ as i64) } < 0 {
                    return Err(io::Error::last_os_error());
                }

                Ok(())
            }
            None => {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "No interface defined"))
            }
        }
    }

    pub fn set_immediate_mode(&self, immediate: bool) -> io::Result<()> {
        let enable: i64 = if immediate { 1 } else { 0 };
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

    pub fn recv(&self) -> io::Result<(i32, Packet)> { //i32 should be the socket address
        if !self.packet_buffer.borrow().is_empty() {
            return Ok((0, self.packet_buffer.borrow_mut().remove(0)));
        }

        let mut buf = vec![0u8; self.buffer_len];

        let len = unsafe { recvfrom(self.fd, buf.as_mut_slice()) };
        if len > 0 {
            let mut offset = 0 as usize;

            let mut ret = None;

            while offset + 18 <= len as usize {
                let tstamp_sec = i32::from_ne_bytes(buf[offset..offset + 4].try_into().unwrap());
                let tstamp_usec = i32::from_ne_bytes(buf[offset + 4..offset + 8].try_into().unwrap());
                let caplen = u32::from_ne_bytes(buf[offset + 8..offset + 12].try_into().unwrap());
                let datalen = u32::from_ne_bytes(buf[offset + 12..offset + 16].try_into().unwrap());
                let hdrlen = u16::from_ne_bytes(buf[offset + 16..offset + 18].try_into().unwrap());

                let packet = Packet::new(self.device.as_ref().unwrap().get_data_link_type(), 0, &buf[offset + hdrlen as usize..(offset + hdrlen as usize + caplen as usize)]);

                match ret {
                    Some(_) => {
                        self.packet_buffer.borrow_mut().push(packet);
                    }
                    None => {
                        ret = Some(packet);
                    }
                }

                let total_len = hdrlen as usize + caplen as usize;
                offset += (total_len + 3) & !3;
            }

            return Ok((0, ret.unwrap()));

        } else if len == -1 {
            let err = io::Error::last_os_error();

            if err.kind() == io::ErrorKind::WouldBlock {
                return Err(io::Error::new(io::ErrorKind::WouldBlock, "No data available"));
            }

            return Err(err);
        }

        Err(io::Error::last_os_error())
    }

    pub fn try_recv(&self) -> io::Result<(i32, Packet)> { //i32 should be the socket address
        if !self.packet_buffer.borrow().is_empty() {
            return Ok((0, self.packet_buffer.borrow_mut().remove(0)));
        }

        let mut readfds: i32 = 0;
        readfds |= 1 << self.fd;

        let mut timeout = TimeVal {
            tv_sec: 0,
            tv_usec: 0
        };

        if unsafe { select(self.fd+1, &mut readfds, ptr::null_mut(), ptr::null_mut(), &mut timeout as *mut TimeVal) } < 0 {
            return Err(io::Error::last_os_error());
        }

        if (readfds & (1 << self.fd)) != 0 {
            return self.recv();
        }

        Err(io::Error::new(io::ErrorKind::WouldBlock, "No data available"))
    }

    pub fn close(&self) {
    }

    pub fn get_device(&self) -> Option<&Device> {
        self.device.as_ref()
    }
}
