use std::any::Any;
use crate::packet::layers::ethernet_frame::ip::udp::dhcp::inter::dhcp_cookie::DHCPCookie;
use crate::packet::layers::inter::layer::Layer;

#[derive(Clone, Debug)]
pub struct DhcpLayer {
    op: u8,           // Message type: 1 = Discover, 2 = Offer, etc.
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
    cookie: DHCPCookie,
    options: Vec<u8>, // DHCP options (e.g., DHCP message type, etc.)
    length: usize
}

impl DhcpLayer {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 239 {
            return None;
        }

        let mut offset = 0;

        let op = buf[offset];
        offset += 1;
        let htype = buf[offset];
        offset += 1;
        let hlen = buf[offset];
        offset += 1;
        let hops = buf[offset];
        offset += 1;
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

        // Read the DHCP cookie (magic cookie)
        let cookie = DHCPCookie::new(buf[offset], buf[offset+1], buf[offset+2], buf[offset+3]);
        offset += 4;

        // Read the DHCP options
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
            cookie,
            options,
            length: buf.len()
        })
    }
}

impl Layer for DhcpLayer {

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();//vec![0; self.len()];

        /*
        buf.push(self.op);
        buf.push(self.htype);
        buf.push(self.hlen);
        buf.push(self.hops);

        buf.extend_from_slice(&self.xid.to_be_bytes());
        buf.extend_from_slice(&self.secs.to_be_bytes());
        buf.extend_from_slice(&self.flags.to_be_bytes());
        buf.extend_from_slice(&self.ciaddr);
        buf.extend_from_slice(&self.yiaddr);
        buf.extend_from_slice(&self.siaddr);
        buf.extend_from_slice(&self.giaddr);
        buf.extend_from_slice(&self.chaddr);
        buf.extend_from_slice(&self.sname);
        buf.extend_from_slice(&self.file);
        buf.extend_from_slice(&self.options);*/

        buf
    }

    fn len(&self) -> usize {
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
