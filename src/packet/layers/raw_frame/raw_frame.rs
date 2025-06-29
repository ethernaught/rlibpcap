use std::any::Any;
use crate::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::packet::layers::ip::ipv6_layer::Ipv6Layer;
use crate::packet::layers::inter::layer::Layer;
use crate::packet::layers::ip::inter::ip_versions::IpVersions;

#[derive(Clone, Debug)]
pub struct RawFrame {
    version: IpVersions,
    data: Option<Box<dyn Layer>>,
    length: usize
}

impl RawFrame {

    pub fn new(version: IpVersions) -> Self {
        Self {
            version,
            data: None,
            length: 0
        }
    }

    pub fn get_version(&self) -> IpVersions {
        self.version
    }

    pub fn set_version(&mut self, version: IpVersions) {
        self.version = version;
    }

    pub fn get_data<T: 'static>(&self) -> Option<&T> {
        self.data.as_ref()?.as_any().downcast_ref::<T>()
    }

    pub fn get_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.data.as_mut()?.as_any_mut().downcast_mut::<T>()
    }

    pub fn set_data(&mut self, data: Box<dyn Layer>) {
        self.length = data.len();
        self.data = Some(data);
    }
}

impl Layer for RawFrame {

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 1 {
            return None;
        }

        let version = IpVersions::from_code((buf[0] >> 4) & 0x0F).unwrap();

        let data = match version {
            IpVersions::Ipv4 => Some(Ipv4Layer::from_bytes(buf).unwrap().upcast()),
            IpVersions::Ipv6 => Some(Ipv6Layer::from_bytes(buf).unwrap().upcast())
        };

        Some(Self {
            version,
            data,
            length: buf.len()
        })
    }

    fn to_bytes(&self) -> Vec<u8> {
        match &self.data {
            Some(data) => data.to_bytes(),
            None => Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn compute_length(&mut self) -> usize {
        self.length = match &self.data {
            Some(layer) => layer.len(),
            None => 0
        };

        self.length
    }

    fn dyn_clone(&self) -> Box<dyn Layer> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
