use std::any::Any;
use std::fmt::Debug;

pub trait Layer: Send + Debug {

    fn from_bytes(buf: &[u8]) -> Option<Self> where Self: Sized;

    fn to_bytes(&self) -> Vec<u8>;

    fn len(&self) -> usize;

    fn compute_length(&mut self) -> usize;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn dyn_clone(&self) -> Box<dyn Layer>;
}

impl Clone for Box<dyn Layer> {

    fn clone(&self) -> Box<dyn Layer> {
        self.dyn_clone()
    }
}
