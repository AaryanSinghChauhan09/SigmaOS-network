#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpProtocol { Tcp, Udp, Icmp }

#[derive(Debug)]
pub struct IpPacketHeader {
    pub protocol: IpProtocol,
    pub source_ip: [u8; 4],
    pub dest_ip: [u8; 4],
}

pub struct NetworkStack;
impl NetworkStack {
    pub fn validate_header(hdr: &IpPacketHeader) -> bool {
        hdr.source_ip != [0, 0, 0, 0] && hdr.dest_ip != [0, 0, 0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_network_packet_header_validation() {
        let hdr = IpPacketHeader {
            protocol: IpProtocol::Tcp,
            source_ip: [192, 168, 1, 5],
            dest_ip: [192, 168, 1, 1],
        };
        assert!(NetworkStack::validate_header(&hdr));
    }
}
