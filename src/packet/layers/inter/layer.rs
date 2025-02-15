use std::any::Any;
use std::fmt::Debug;

pub trait Layer: Send + Debug {

    fn len(&self) -> usize;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn dyn_clone(&self) -> Box<dyn Layer>;
}
