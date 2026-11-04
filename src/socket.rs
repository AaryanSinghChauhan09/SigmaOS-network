//! Socket management for network operations
//! Follows: Network Abstraction, Component Isolation

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/// Socket configuration
#[derive(Debug, Clone)]
pub struct SocketConfig {
    pub bind_address: IpAddr,
    pub port: u16,
    pub backlog: u32,
}

impl Default for SocketConfig {
    fn default() -> Self {
        SocketConfig {
            bind_address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port: 8080,
            backlog: 128,
        }
    }
}

/// Network socket abstraction
pub struct Socket {
    config: SocketConfig,
    is_bound: bool,
}

impl Socket {
    pub fn new(config: SocketConfig) -> Self {
        Socket {
            config,
            is_bound: false,
        }
    }

    /// Bind socket to address
    pub fn bind(&mut self) -> Result<(), &'static str> {
        if self.is_bound {
            return Err("Socket already bound");
        }
        self.is_bound = true;
        log::info!("Socket bound to {}:{}", self.config.bind_address, self.config.port);
        Ok(())
    }

    /// Listen for connections
    pub fn listen(&self) -> Result<(), &'static str> {
        if !self.is_bound {
            return Err("Socket not bound");
        }
        log::info!("Listening with backlog: {}", self.config.backlog);
        Ok(())
    }

    /// Accept incoming connection
    pub fn accept(&self) -> Result<SocketAddr, &'static str> {
        if !self.is_bound {
            return Err("Socket not bound");
        }
        Ok(SocketAddr::new(self.config.bind_address, self.config.port))
    }

    /// Close socket
    pub fn close(&mut self) {
        self.is_bound = false;
        log::info!("Socket closed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_binding() {
        let config = SocketConfig::default();
        let mut socket = Socket::new(config);
        assert!(socket.bind().is_ok());
        assert!(socket.listen().is_ok());
    }
}
