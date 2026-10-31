#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]
#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── WireGuard Peer Session State ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgSessionState {
    Unauthenticated,
    HandshakeInitiated,
    Established,
    Rekeying,
}

// ─── WireGuard Peer Descriptor ────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WgPeer {
    pub public_key: [u8; 32],
    pub endpoint: String,
    pub allowed_ips: Vec<String>, // CIDR list, e.g. "10.0.0.2/32"
    pub session_state: WgSessionState,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub last_handshake_tick: u64,
    pub sender_index: u32,
    pub receiver_index: u32,
}

impl WgPeer {
    pub fn new(public_key: [u8; 32], endpoint: &str, allowed_ip: &str) -> Self {
        WgPeer {
            public_key,
            endpoint: endpoint.to_string(),
            allowed_ips: vec![allowed_ip.to_string()],
            session_state: WgSessionState::Unauthenticated,
            tx_bytes: 0,
            rx_bytes: 0,
            last_handshake_tick: 0,
            sender_index: 0,
            receiver_index: 0,
        }
    }
}

// ─── Sovereign WireGuard Tunnel Interface ─────────────────────────────────────

pub struct SovereignWireGuardTunnel {
    pub ifname: String,
    pub private_key: [u8; 32],
    pub public_key: [u8; 32],
    pub listen_port: u16,
    pub peers: Vec<WgPeer>,
    pub next_index: u32,
    pub packets_tunneled: u64,
    pub packets_dropped: u64,
}

impl SovereignWireGuardTunnel {
    pub fn new(ifname: &str, listen_port: u16) -> Self {
        SovereignWireGuardTunnel {
            ifname: ifname.to_string(),
            // No cryptographic key generation is available. Zero values are
            // placeholders and all handshake/transport operations fail closed.
            private_key: [0; 32],
            public_key: [0; 32],
            listen_port,
            peers: Vec::new(),
            next_index: 100,
            packets_tunneled: 0,
            packets_dropped: 0,
        }
    }

    pub fn add_peer(&mut self, peer: WgPeer) {
        self.peers.push(peer);
    }

    /// Initiate a WireGuard handshake. Unavailable until a real Noise backend exists.
    pub fn initiate_handshake(
        &mut self,
        _peer_key: &[u8; 32],
        _current_tick: u64,
    ) -> Result<u32, &'static str> {
        Err("WireGuard cryptographic handshake backend is unavailable")
    }

    /// Complete a WireGuard handshake. Unavailable until a real Noise backend exists.
    pub fn complete_handshake(
        &mut self,
        _peer_key: &[u8; 32],
        _receiver_idx: u32,
    ) -> Result<(), &'static str> {
        Err("WireGuard cryptographic handshake backend is unavailable")
    }

    /// Encrypt and route an outgoing packet. Unavailable until authenticated
    /// WireGuard transport encryption is implemented.
    pub fn encapsulate_and_send(
        &mut self,
        _dest_ip: &str,
        _payload: &[u8],
    ) -> Result<Vec<u8>, &'static str> {
        self.packets_dropped = self.packets_dropped.saturating_add(1);
        Err("WireGuard authenticated transport backend is unavailable")
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wg_tunnel_creation() {
        let tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        assert_eq!(tunnel.ifname, "wg0");
        assert_eq!(tunnel.listen_port, 51820);
        assert_eq!(tunnel.peers.len(), 0);
    }

    #[test]
    fn test_wg_cryptokey_routing_add_peer() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let peer_key = [0x42; 32];
        let peer = WgPeer::new(peer_key, "192.168.1.50:51820", "10.0.0.2");
        tunnel.add_peer(peer);
        assert_eq!(tunnel.peers.len(), 1);
    }

    #[test]
    fn test_wg_handshake_lifecycle() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let peer_key = [0xAA; 32];
        let peer = WgPeer::new(peer_key, "10.10.10.1:51820", "10.0.0.5");
        tunnel.add_peer(peer);

        assert!(tunnel.initiate_handshake(&peer_key, 1000).is_err());
        assert!(tunnel.complete_handshake(&peer_key, 1001).is_err());
        assert_eq!(
            tunnel.peers[0].session_state,
            WgSessionState::Unauthenticated
        );
    }

    #[test]
    fn test_wg_encapsulate_unauthenticated_fails() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let peer_key = [0xBB; 32];
        let peer = WgPeer::new(peer_key, "10.0.0.1:51820", "10.0.0.3");
        tunnel.add_peer(peer);

        // Not established -> should fail
        let res = tunnel.encapsulate_and_send("10.0.0.3", b"ping");
        assert!(res.is_err());
        assert_eq!(tunnel.packets_dropped, 1);
    }

    #[test]
    fn test_wg_encapsulate_fails_closed() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let peer_key = [0xCC; 32];
        let peer = WgPeer::new(peer_key, "10.0.0.1:51820", "10.0.0.4");
        tunnel.add_peer(peer);
        assert!(tunnel.initiate_handshake(&peer_key, 100).is_err());
        assert!(tunnel.complete_handshake(&peer_key, 555).is_err());
        assert!(tunnel
            .encapsulate_and_send("10.0.0.4", b"hello wireguard")
            .is_err());
        assert_eq!(tunnel.packets_tunneled, 0);
    }

    #[test]
    fn test_wg_routing_miss_fails() {
        let mut tunnel = SovereignWireGuardTunnel::new("wg0", 51820);
        let res = tunnel.encapsulate_and_send("192.168.99.99", b"unroutable");
        assert!(res.is_err());
    }
}
