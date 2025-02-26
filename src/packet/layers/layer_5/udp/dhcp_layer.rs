use std::any::Any;
use crate::packet::layers::inter::layer::Layer;

#[derive(Clone, Debug)]
pub struct DhcpLayer {
    op: u8,               // OP: 1 = Request, 2 = Reply
    htype: u8,            // Hardware type: 1 = Ethernet
    hlen: u8,             // Hardware address length: 6 for Ethernet
    hops: u8,
    xid: u32,             // Transaction ID
    secs: u16,            // Seconds elapsed
    flags: u16,           // Flags: 0 = Broadcast, 1 = Unicast
    ciaddr: [u8; 4],      // Client IP address (0.0.0.0 for discover)
    yiaddr: [u8; 4],      // Your (client) IP address (assigned by server)
    siaddr: [u8; 4],      // Server IP address
    giaddr: [u8; 4],      // Gateway IP address
    chaddr: [u8; 16],     // Client hardware address (MAC)
    sname: [u8; 64],      // Server host name
    file: [u8; 128],      // Boot file name
    options: [u8; 312],   // DHCP options
    length: usize
}

impl DhcpLayer {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 239 {
            return None;
        }

        let op = buf[0];
        let htype = buf[1];
        let hlen = buf[2];
        let hops = buf[3];
        let mut off = 3;

        let xid = u32::from_be_bytes(buf[off..off + 4].try_into().unwrap());
        off += 4;
        let secs = u16::from_be_bytes(buf[off..off + 2].try_into().unwrap());
        off += 2;
        let flags = u16::from_be_bytes(buf[off..off + 2].try_into().unwrap());
        off += 2;

        let ciaddr = buf[off..off + 4].try_into().unwrap();
        off += 4;
        let yiaddr = buf[off..off + 4].try_into().unwrap();
        off += 4;
        let siaddr = buf[off..off + 4].try_into().unwrap();
        off += 4;
        let giaddr = buf[off..off + 4].try_into().unwrap();
        off += 4;

        let chaddr = buf[off..off + 16].try_into().unwrap();
        off += 16;

        let sname = buf[off..off + 64].try_into().unwrap();
        off += 64;
        let file = buf[off..off + 128].try_into().unwrap();
        off += 128;

        let options = buf[off..off + 312].try_into().unwrap();
        off += 312;

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
}

impl Layer for DhcpLayer {

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();//vec![0; self.len()];

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
        buf.extend_from_slice(&self.options);

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
