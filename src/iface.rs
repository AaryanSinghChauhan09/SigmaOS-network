//! Network interface: MAC address, IP assignment, MTU.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacAddr([u8; 6]);

impl MacAddr {
    pub fn new(a: [u8; 6]) -> Self { Self(a) }
    pub fn is_broadcast(&self) -> bool { self.0 == [0xFF; 6] }
}

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
    pub mac: MacAddr,
    pub ips: Vec<super::ip::IpAddr>,
    pub mtu: u32,
    pub up: bool,
}

impl Interface {
    pub fn new(name: String, mac: MacAddr) -> Self {
        Self { name, mac, ips: vec![], mtu: 1500, up: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_addr() {
        let m = MacAddr::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(!m.is_broadcast());
        assert!(MacAddr::new([0xFF; 6]).is_broadcast());
    }
}
