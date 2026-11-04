mod config;
mod error;
mod service;
mod socket;
mod packet;
mod connection;

use config::Config;
use service::Service;
use socket::{Socket, SocketConfig};
use packet::Packet;
use connection::{Connection, ConnectionState};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    
    let config = Config::default();
    let mut service = Service::new(config);
    
    service.start()?;
    println!("Service started successfully");
    
    // Demonstrate socket management
    let socket_config = SocketConfig::default();
    let mut socket = Socket::new(socket_config);
    socket.bind()?;
    socket.listen()?;
    println!("Socket bound and listening");
    
    // Demonstrate packet processing
    let data = vec![1, 2, 3, 4, 5];
    let packet = Packet::new(data);
    println!("Packet processed: {} bytes", packet.len());
    
    // Demonstrate connection management
    let local_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let remote_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9090);
    let mut conn = Connection::new(local_addr);
    conn.connect(remote_addr)?;
    println!("Connection state: {:?}", conn.state());
    
    service.stop()?;
    println!("Service stopped successfully");
    
    Ok(())
}
