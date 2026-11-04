//! Packet processing and management
//! Follows: Zero-Copy, Performance Optimization

use std::slice;

/// Network packet structure
#[derive(Debug, Clone)]
pub struct Packet {
    data: Vec<u8>,
    offset: usize,
    length: usize,
}

impl Packet {
    pub fn new(data: Vec<u8>) -> Self {
        let length = data.len();
        Packet {
            data,
            offset: 0,
            length,
        }
    }

    /// Get packet data slice (zero-copy)
    pub fn as_slice(&self) -> &[u8] {
        &self.data[self.offset..self.offset + self.length]
    }

    /// Get mutable slice (zero-copy)
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data[self.offset..self.offset + self.length]
    }

    /// Get packet length
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if packet is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Get header as slice
    pub fn header(&self, header_size: usize) -> &[u8] {
        &self.data[self.offset..self.offset + header_size.min(self.length)]
    }

    /// Get payload as slice
    pub fn payload(&self, header_size: usize) -> &[u8] {
        let payload_start = self.offset + header_size.min(self.length);
        &self.data[payload_start..self.offset + self.length]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_processing() {
        let data = vec![1, 2, 3, 4, 5];
        let packet = Packet::new(data);
        assert_eq!(packet.len(), 5);
        assert!(!packet.is_empty());
        assert_eq!(packet.header(2), &[1, 2]);
        assert_eq!(packet.payload(2), &[3, 4, 5]);
    }
}
