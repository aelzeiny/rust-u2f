use std::io;
use u2fhid_protocol::{Packet, U2fHidServer};
use std::collections::VecDeque;

pub struct PiU2fAdapter<D> 
where 
    D: PiHidDevice,
{
    device: D,
    server: SimplifiedServer,
}

// Simple server implementation that processes U2F packets
pub struct SimplifiedServer {
    pending_responses: VecDeque<Packet>,
}

impl SimplifiedServer {
    pub fn new() -> Self {
        Self {
            pending_responses: VecDeque::new(),
        }
    }
    
    pub fn process_packet(&mut self, packet: Packet) -> Vec<Packet> {
        // TODO: Implement actual packet processing logic using u2f_core
        // For now, just echo back the packet as a placeholder
        vec![packet]
    }
}

impl<D> PiU2fAdapter<D> 
where 
    D: PiHidDevice,
{
    pub fn new(device: D, server: SimplifiedServer) -> Self {
        Self { device, server }
    }
    
    pub fn run(&mut self) -> io::Result<()> {
        let mut input_buf = [0u8; 64];
        
        loop {
            // Read HID report from the host
            let read_size = self.device.read_report(&mut input_buf)?;
            
            if read_size > 0 {
                // Process the incoming packet
                if let Ok(packet) = Packet::from_bytes(&input_buf) {
                    // Let the U2F server process the packet
                    let responses = self.server.process_packet(packet);
                    
                    // Send any response packets back to the host
                    for response_packet in responses {
                        let response_bytes = response_packet.to_bytes();
                        self.device.write_report(&response_bytes)?;
                    }
                }
            }
        }
    }
}

pub trait PiHidDevice {
    fn read_report(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn write_report(&mut self, data: &[u8]) -> io::Result<usize>;
}

impl PiHidDevice for pi_hid::PiHidDevice {
    fn read_report(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_report(buf)
    }
    
    fn write_report(&mut self, data: &[u8]) -> io::Result<usize> {
        self.write_report(data)
    }
}

#[cfg(test)]
mod tests;