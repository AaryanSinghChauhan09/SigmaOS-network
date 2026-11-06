//! IP layer: IPv4/IPv6 headers and packet model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpVersion { V4, V6 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpAddr {
    V4([u8; 4]),
    V6([u8; 16]),
}

impl IpAddr {
    pub fn is_v4(&self) -> bool { matches!(self, Self::V4(_)) }
    pub fn is_v6(&self) -> bool { matches!(self, Self::V6(_)) }
    pub fn unspecified(&self) -> bool {
        match self {
            Self::V4(a) => *a == [0, 0, 0, 0],
            Self::V6(a) => *a == [0; 16],
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpHeader {
    pub version: IpVersion,
    pub ttl: u8,
    pub protocol: u8,
    pub src: IpAddr,
    pub dst: IpAddr,
}

#[derive(Debug, Clone)]
pub struct IpPacket {
    pub header: IpHeader,
    pub payload: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ip_addr_v4() {
        let a = IpAddr::V4([192, 168, 1, 5]);
        assert!(a.is_v4());
        assert!(!a.is_v6());
        assert!(!a.unspecified());
    }
}
