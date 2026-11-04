//! Connection management with state tracking
//! Follows: State Management, Reliability

use std::net::SocketAddr;

/// Connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
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

/// Network connection
pub struct Connection {
    local_addr: SocketAddr,
    remote_addr: Option<SocketAddr>,
    state: ConnectionState,
}

impl Connection {
    pub fn new(local_addr: SocketAddr) -> Self {
        Connection {
            local_addr,
            remote_addr: None,
            state: ConnectionState::Closed,
        }
    }

    /// Establish connection
    pub fn connect(&mut self, remote_addr: SocketAddr) -> Result<(), &'static str> {
        if self.state != ConnectionState::Closed {
            return Err("Connection not in closed state");
        }
        self.remote_addr = Some(remote_addr);
        self.state = ConnectionState::SynSent;
        log::info!("Connecting to {}", remote_addr);
        Ok(())
    }

    /// Accept connection
    pub fn accept(&mut self, remote_addr: SocketAddr) -> Result<(), &'static str> {
        if self.state != ConnectionState::Listen {
            return Err("Connection not in listen state");
        }
        self.remote_addr = Some(remote_addr);
        self.state = ConnectionState::SynReceived;
        log::info!("Accepting connection from {}", remote_addr);
        Ok(())
    }

    /// Close connection
    pub fn close(&mut self) -> Result<(), &'static str> {
        if self.state == ConnectionState::Closed {
            return Err("Connection already closed");
        }
        self.state = ConnectionState::FinWait1;
        log::info!("Closing connection");
        Ok(())
    }

    /// Get connection state
    pub fn state(&self) -> ConnectionState {
        self.state.clone()
    }

    /// Check if connection is established
    pub fn is_established(&self) -> bool {
        self.state == ConnectionState::Established
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_lifecycle() {
        let local = "127.0.0.1:8080".parse().unwrap();
        let remote = "127.0.0.1:9090".parse().unwrap();
        let mut conn = Connection::new(local);
        
        assert_eq!(conn.state(), ConnectionState::Closed);
        assert!(conn.connect(remote).is_ok());
        assert_eq!(conn.state(), ConnectionState::SynSent);
    }
}
