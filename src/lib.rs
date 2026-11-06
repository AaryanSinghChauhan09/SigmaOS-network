//! SigmaOS-network — network stack model.
//!
//! Models a Unix-style network stack: IPv4/IPv6 packet formats, TCP/UDP
//! transport layers, socket model, routing table, network interfaces,
//! and ARP resolution. Deterministic and std-only (no actual syscalls).

pub mod ip;
pub mod tcp;
pub mod udp;
pub mod socket;
pub mod route;
pub mod iface;
pub mod arp;

pub use ip::{IpAddr, IpHeader, IpPacket, IpVersion};
pub use tcp::{TcpHeader, TcpFlags, TcpSegment};
pub use udp::{UdpHeader, UdpPacket};
pub use socket::{Socket, SocketState, SocketKind};
pub use route::{Route, RouteTable};
pub use iface::{Interface, MacAddr};
pub use arp::{ArpEntry, ArpTable};
