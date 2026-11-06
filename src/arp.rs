//! ARP table: IPv4 ↔ MAC resolution.

#[derive(Debug, Clone)]
pub struct ArpEntry {
    pub ip: [u8; 4],
    pub mac: crate::iface::MacAddr,
}

#[derive(Debug, Default)]
pub struct ArpTable {
    entries: Vec<ArpEntry>,
}

impl ArpTable {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, e: ArpEntry) { self.entries.push(e); }
    pub fn resolve(&self, ip: [u8; 4]) -> Option<&crate::iface::MacAddr> {
        self.entries.iter().find(|e| e.ip == ip).map(|e| &e.mac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arp_resolve() {
        let mut t = ArpTable::new();
        t.add(ArpEntry { ip: [192, 168, 1, 1], mac: crate::iface::MacAddr::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]) });
        assert!(t.resolve([192, 168, 1, 1]).is_some());
    }
}
