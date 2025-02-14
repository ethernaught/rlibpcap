pub mod capture;

#[cfg(test)]
mod tests {
    use crate::capture::Capture;

    #[test]
    fn it_works() {
        let interface = "wlp7s0";
        let mut cap = Capture::from_device(interface).unwrap();
        cap.open().unwrap();


        while let Ok(packet) = cap.next_packet() {
            println!("Received packet: {} bytes", packet.data().len());
            println!("{:x?}", packet.data());
        }
    }
}
