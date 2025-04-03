use std::any::Any;
use std::fmt::Debug;

pub trait Layer: Send + Debug where Self: 'static {

    fn from_bytes(buf: &[u8]) -> Option<Self> where Self: Sized;

    fn to_bytes(&self) -> Vec<u8>;

    fn len(&self) -> usize;

    fn compute_length(&mut self) -> usize;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn dyn_clone(&self) -> Box<dyn Layer>;

    fn upcast_ref(&self) -> &dyn Layer
    where
        Self: Sized
    {
        self
    }

    fn upcast(self) -> Box<dyn Layer>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl Clone for Box<dyn Layer> {

    fn clone(&self) -> Box<dyn Layer> {
        self.dyn_clone()
    }
}
