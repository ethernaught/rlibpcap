
pub fn calculate_checksum(buf: &[u8]) -> u16 {
    let mut sum: u32 = 0;

    for i in (0..buf.len()).step_by(2) {
        let word = if i + 1 < buf.len() {
            (buf[i] as u16) << 8 | (buf[i + 1] as u16)
        } else {
            (buf[i] as u16) << 8 // Last odd byte
        };
        sum += word as u32;
    }

    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    !(sum as u16)
}
