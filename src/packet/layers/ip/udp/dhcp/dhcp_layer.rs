use std::any::Any;
use crate::packet::layers::ip::udp::dhcp::inter::dhcp_operations::DhcpOperations;
use crate::packet::layers::inter::layer::Layer;

pub const DHCP_COOKIE: u32 = 0x63825363;

#[derive(Clone, Debug)]
pub struct DhcpLayer {
    op: DhcpOperations,           // Message type: 1 = Discover, 2 = Offer, etc.
    htype: u8,        // Hardware type (1 = Ethernet)
    hlen: u8,         // Hardware address length (6 for Ethernet)
    hops: u8,         // Number of hops
    xid: u32,         // Transaction ID
    secs: u16,        // Seconds elapsed
    flags: u16,       // Flags
    ciaddr: u32,      // Client IP address
    yiaddr: u32,      // Your IP address (offered by DHCP server)
    siaddr: u32,      // Server IP address
    giaddr: u32,      // Gateway IP address
    chaddr: [u8; 16], // Client hardware address
    sname: [u8; 64],  // Server name
    file: [u8; 128],  // Boot file name
    options: Vec<u8>, // DHCP options (e.g., DHCP message type, etc.)
    length: usize
}

impl DhcpLayer {
}

impl Layer for DhcpLayer {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 239 {
            return None;
        }

        let op = DhcpOperations::from_code(buf[0]).unwrap();
        let htype = buf[1];
        let hlen = buf[2];
        let hops = buf[3];
        let mut offset = 4;

        let xid = u32::from_be_bytes(buf[offset..offset + 4].try_into().ok()?);
        offset += 4;
        let secs = u16::from_be_bytes(buf[offset..offset + 2].try_into().ok()?);
        offset += 2;
        let flags = u16::from_be_bytes(buf[offset..offset + 2].try_into().ok()?);
        offset += 2;
        let ciaddr = u32::from_be_bytes(buf[offset..offset + 4].try_into().ok()?);
        offset += 4;
        let yiaddr = u32::from_be_bytes(buf[offset..offset + 4].try_into().ok()?);
        offset += 4;
        let siaddr = u32::from_be_bytes(buf[offset..offset + 4].try_into().ok()?);
        offset += 4;
        let giaddr = u32::from_be_bytes(buf[offset..offset + 4].try_into().ok()?);
        offset += 4;

        let mut chaddr = [0u8; 16];
        chaddr.copy_from_slice(&buf[offset..offset + 16]);
        offset += 16;

        let mut sname = [0u8; 64];
        sname.copy_from_slice(&buf[offset..offset + 64]);
        offset += 64;

        let mut file = [0u8; 128];
        file.copy_from_slice(&buf[offset..offset + 128]);
        offset += 128;

        if DHCP_COOKIE != u32::from_be_bytes([buf[offset], buf[offset+1], buf[offset+2], buf[offset+3]]) {
            return None;
        }

        offset += 4;

        let mut options = Vec::new();
        while offset < buf.len() {
            options.push(buf[offset]);
            offset += 1;
        }

        Some(Self {
            op,
            htype,
            hlen,
            hops,
            xid,
            secs,
            flags,
            ciaddr,
            yiaddr,
            siaddr,
            giaddr,
            chaddr,
            sname,
            file,
            options,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; self.length];

        buf[0] = self.op.get_code();
        buf[1] = self.htype;
        buf[2] = self.hlen;
        buf[3] = self.hops;

        let mut off = 4;

        buf[off..off + 4].copy_from_slice(&self.xid.to_be_bytes());
        off += 4;
        buf[off..off + 2].copy_from_slice(&self.secs.to_be_bytes());
        off += 2;
        buf[off..off + 2].copy_from_slice(&self.flags.to_be_bytes());
        off += 2;
        buf[off..off + 4].copy_from_slice(&self.ciaddr.to_be_bytes());
        off += 4;
        buf[off..off + 4].copy_from_slice(&self.yiaddr.to_be_bytes());
        off += 4;
        buf[off..off + 4].copy_from_slice(&self.siaddr.to_be_bytes());
        off += 4;
        buf[off..off + 4].copy_from_slice(&self.giaddr.to_be_bytes());
        off += 4;

        buf[off..off + self.chaddr.len()].copy_from_slice(&self.chaddr);
        off += self.chaddr.len();
        buf[off..off + self.sname.len()].copy_from_slice(&self.sname);
        off += self.sname.len();
        buf[off..off + self.file.len()].copy_from_slice(&self.file);
        off += self.file.len();
        buf[off..off + 4].copy_from_slice(&DHCP_COOKIE.to_be_bytes());
        off += 4;
        buf[off..off + self.options.len()].copy_from_slice(&self.options);

        buf
    }

    fn len(&self) -> usize {
        self.length
    }

    fn compute_length(&mut self) -> usize {
        self.length = 240 + self.options.len();
        self.length
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Layer> {
        Box::new(self.clone())
    }
}
