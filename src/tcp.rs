//! TCP layer: header, flags, segment model.

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TcpFlags: u8 {
        const SYN = 0b00000010;
        const ACK = 0b00010000;
        const FIN = 0b00000001;
        const RST = 0b00000100;
        const PSH = 0b00001000;
    }
}

#[derive(Debug, Clone)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: TcpFlags,
    pub window: u16,
}

#[derive(Debug, Clone)]
pub struct TcpSegment {
    pub header: TcpHeader,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tcp_flags() {
        let f = TcpFlags::SYN | TcpFlags::ACK;
        assert!(f.contains(TcpFlags::SYN));
        assert!(f.contains(TcpFlags::ACK));
    }
}
