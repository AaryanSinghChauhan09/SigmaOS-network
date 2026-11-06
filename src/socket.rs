//! Socket model: state machine and kinds.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketKind { Stream, Datagram, Raw }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

#[derive(Debug, Clone)]
pub struct Socket {
    pub kind: SocketKind,
    pub state: SocketState,
    pub local_addr: (u16, crate::ip::IpAddr),
    pub remote_addr: Option<(u16, crate::ip::IpAddr)>,
}

impl Socket {
    pub fn new(kind: SocketKind) -> Self {
        Self { kind, state: SocketState::Closed, local_addr: (0, crate::ip::IpAddr::V4([0,0,0,0])), remote_addr: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_state_transitions() {
        let mut s = Socket::new(SocketKind::Stream);
        s.state = SocketState::Established;
        assert_eq!(s.state, SocketState::Established);
    }
}
