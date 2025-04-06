use crate::windows::devices::Device;

#[derive(Debug, Clone)]
pub struct Capture {
    //fd: RawFd,
    device: Option<Device>
}
