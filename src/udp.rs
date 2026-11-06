//! UDP layer: header and packet model.

#[derive(Debug, Clone)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
}

#[derive(Debug, Clone)]
pub struct UdpPacket {
    pub header: UdpHeader,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn udp_packet() {
        let p = UdpPacket {
            header: UdpHeader { src_port: 1234, dst_port: 80, length: 8 },
            data: vec![],
        };
        assert_eq!(p.header.dst_port, 80);
    }
}
