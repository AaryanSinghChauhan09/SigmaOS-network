//! Routing table: prefixes, next hops, metrics.

#[derive(Debug, Clone)]
pub struct Route {
    pub dest: crate::ip::IpAddr,
    pub prefix_len: u8,
    pub gateway: Option<crate::ip::IpAddr>,
    pub metric: u32,
    pub iface: String,
}

#[derive(Debug, Default)]
pub struct RouteTable {
    routes: Vec<Route>,
}

impl RouteTable {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, r: Route) { self.routes.push(r); }
    pub fn lookup(&self, dst: &crate::ip::IpAddr) -> Option<&Route> {
        self.routes.iter().find(|r| {
            let dest = &r.dest;
            let mask_len = r.prefix_len as usize;
            match (dest, dst) {
                (crate::ip::IpAddr::V4(a), crate::ip::IpAddr::V4(b)) => {
                    let mask = if mask_len >= 32 { 0xFFFFFFFFu32 } else { 0xFFFFFFFFu32 << (32 - mask_len) };
                    let a_u = u32::from_be_bytes(*a);
                    let b_u = u32::from_be_bytes(*b);
                    (a_u & mask) == (b_u & mask)
                }
                _ => false,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_lookup() {
        let mut rt = RouteTable::new();
        rt.add(Route {
            dest: crate::ip::IpAddr::V4([192, 168, 1, 0]),
            prefix_len: 24,
            gateway: None,
            metric: 0,
            iface: "eth0".into(),
        });
        assert!(rt.lookup(&crate::ip::IpAddr::V4([192, 168, 1, 5])).is_some());
    }
}
